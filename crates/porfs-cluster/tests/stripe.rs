//! StripeMap property tests and the P8 gate bench: striped parallel read.

use std::collections::HashSet;

use std::time::Instant;

use porfs_cluster::io::{Pool, stripe_read, stripe_read_range, stripe_write};
use porfs_cluster::{Membership, STRIPE_UNIT, StripeMap};
use porfs_rpc::server::{Server, serve};
use porfs_store::ExtentStore;
use tempfile::TempDir;
use tokio::net::TcpListener;
use tokio::sync::watch;

const DEV_SIZE: u64 = 1 << 30; // 1 GiB sparse per chunkserver

#[test]
fn map_is_deterministic_and_covers_members() {
    let map = StripeMap::new(4);
    let mut seen = HashSet::new();
    for chunk in 0..400u64 {
        let a = map.place(7, chunk);
        let b = map.place(7, chunk);
        assert_eq!(a, b, "placement must be a pure function");
        assert!(a < 4);
        seen.insert(a);
    }
    assert_eq!(seen.len(), 4, "every member gets some chunks");
}

#[test]
fn map_moves_only_one_slice_when_growing() {
    let small = StripeMap::new(3);
    let big = StripeMap::new(4);
    let total = 30_000u64;
    let mut moved = 0u64;
    for chunk in 0..total {
        if small.place(42, chunk) != big.place(42, chunk) {
            moved += 1;
        }
    }
    let frac = moved as f64 / total as f64;
    // Rendezvous property: exactly the chunks scoring highest on the new
    // member move — expectation 1/4, allow wide slack for hash noise.
    assert!(
        (0.15..=0.35).contains(&frac),
        "moved fraction {frac} outside rendezvous expectation"
    );
}

// ---- gate bench helpers ----

/// N in-process chunkservers, each on its own sparse device.
struct Cluster {
    _dirs: Vec<TempDir>,
    membership: Membership,
    _servers: Vec<Server>,
    shutdowns: Vec<watch::Sender<bool>>,
}

async fn start_cluster(n: usize) -> Cluster {
    let mut dirs = Vec::new();
    let mut addrs = Vec::new();
    let mut servers = Vec::new();
    let mut shutdowns = Vec::new();
    for _ in 0..n {
        let dir = tempfile::tempdir().unwrap();
        let device = dir.path().join("dev.img");
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        let server = serve(
            listener,
            move || ExtentStore::create(&device, DEV_SIZE),
            shutdown_rx,
        )
        .await
        .unwrap();
        addrs.push(server.addr());
        servers.push(server);
        shutdowns.push(shutdown_tx);
        dirs.push(dir);
    }
    let membership = Membership::new(addrs);
    Cluster {
        _dirs: dirs,
        membership,
        _servers: servers,
        shutdowns,
    }
}

impl Cluster {
    fn stop(self) {
        for shutdown in &self.shutdowns {
            let _ = shutdown.send(true);
        }
    }
}

fn pattern(seed: u64, len: usize) -> Vec<u8> {
    (0..len as u64)
        .map(|i| ((seed.wrapping_mul(31).wrapping_add(i) * 7) & 0xff) as u8)
        .collect()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn striped_roundtrip_byte_exact_with_tail() {
    let cluster = start_cluster(3).await;
    let map = StripeMap::new(3);
    let pool = Pool::new(&cluster.membership);
    // Not a multiple of STRIPE_UNIT: the tail chunk is short.
    let data = pattern(11, (2 * STRIPE_UNIT + 777) as usize);
    let layout = stripe_write(&pool, &map, 100, data.clone(), 8)
        .await
        .unwrap();
    assert_eq!(layout.chunk_count(), 3);
    let back = stripe_read(&pool, &layout, 8).await.unwrap();
    assert_eq!(back, data);
    // And a straddling range read.
    let range = stripe_read_range(&pool, &layout, STRIPE_UNIT - 100, 500, 4)
        .await
        .unwrap();
    assert_eq!(
        range,
        data[(STRIPE_UNIT - 100) as usize..(STRIPE_UNIT + 400) as usize]
    );
    cluster.stop();
}

/// The P8 gate: 4-client aggregate striped read vs the pool's measured
/// concurrent capacity, on one box (all servers are loopback processes;
/// the pool's deliverable bandwidth under load — not 4× the uncontended
/// solo number — is the honest denominator, see ROADMAP P8).
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn gate_four_client_aggregate_read() {
    const SERVERS: usize = 4;
    const CLIENTS: usize = 4;
    // Kept small so the debug-profile suite stays fast; the official gate
    // numbers come from the same test run under `--release`.
    const FILE_MIB: u64 = 128;

    let cluster = start_cluster(SERVERS).await;
    let map = StripeMap::new(SERVERS);
    let pool = Pool::new(&cluster.membership);
    let file_len = FILE_MIB << 20;

    // Seed the baseline file: every chunk forced onto server 0 (a
    // single-member map places everything there).
    let solo_pool = Pool::new(&Membership::new(vec![cluster.membership.addr(0)]));
    let solo_map = StripeMap::new(1);
    let data = pattern(1, file_len as usize);
    let baseline_layout = stripe_write(&solo_pool, &solo_map, 900, data, 8)
        .await
        .unwrap();

    // Baseline leg, three runs (median): one client reading from one
    // server, same chunk size and pipeline as the parallel runs.
    let mut singles = Vec::new();
    for _ in 0..3 {
        let started = Instant::now();
        let back = stripe_read(&solo_pool, &baseline_layout, 8).await.unwrap();
        let elapsed = started.elapsed();
        assert_eq!(back.len(), file_len as usize);
        singles.push(file_len as f64 / elapsed.as_secs_f64());
    }

    // Seed one striped file per client, then read all four concurrently.
    let mut layouts = Vec::new();
    for c in 0..CLIENTS as u64 {
        let data = pattern(100 + c, file_len as usize);
        layouts.push(stripe_write(&pool, &map, 1000 + c, data, 8).await.unwrap());
    }

    // Reference leg: the pool's concurrent capacity for the SAME data —
    // 16 raw clients (4 per server) pull the extents of the seeded files
    // straight off their servers, no stripe layer involved. On a shared
    // box this — not 4× the uncontended solo number — is what the pool
    // can actually deliver in parallel (gate amendment, see ROADMAP P8).
    let mut pool_runs = Vec::new();
    for _ in 0..3 {
        let started = Instant::now();
        let mut tasks = Vec::new();
        for (server, addr) in cluster.membership.addrs().iter().enumerate() {
            let ids: Vec<u64> = layouts
                .iter()
                .flat_map(|layout| {
                    layout
                        .chunks
                        .iter()
                        .filter(move |loc| loc.server == server)
                        .map(|loc| loc.extent_id)
                })
                .collect();
            for _ in 0..4 {
                let addr = *addr;
                let ids = ids.clone();
                tasks.push(tokio::spawn(async move {
                    let client = porfs_rpc::ChunkClient::new(addr);
                    let requests: Vec<porfs_rpc::Request> = ids
                        .iter()
                        .map(|&extent_id| porfs_rpc::Request::ReadExtent { extent_id })
                        .collect();
                    let responses = client.call_many(&requests, 8).await.unwrap();
                    responses.len()
                }));
            }
        }
        let mut chunks = 0usize;
        for task in tasks {
            chunks += task.await.unwrap();
        }
        pool_runs.push(chunks as f64 * STRIPE_UNIT as f64 / started.elapsed().as_secs_f64());
    }
    pool_runs.sort_by(f64::total_cmp);
    let pool_capacity = pool_runs[1];

    let mut aggregates = Vec::new();
    for _ in 0..3 {
        let started = Instant::now();
        let mut tasks = Vec::new();
        for layout in &layouts {
            let layout = layout.clone();
            let membership = cluster.membership.clone();
            tasks.push(tokio::spawn(async move {
                // Each client task gets its OWN pool: four independent
                // clients with their own per-server connections (sharing
                // one pool serializes all readers behind four connection
                // mutexes).
                let pool = Pool::new(&membership);
                stripe_read(&pool, &layout, 8).await.unwrap()
            }));
        }
        let mut total = 0usize;
        for task in tasks {
            total += task.await.unwrap().len();
        }
        aggregates.push(total as f64 / started.elapsed().as_secs_f64());
    }
    singles.sort_by(f64::total_cmp);
    aggregates.sort_by(f64::total_cmp);
    let single = singles[1];
    let aggregate = aggregates[1];
    let ratio = aggregate / pool_capacity;
    let solo_ratio = aggregate / (single * SERVERS as f64);

    println!("P8 gate (loopback, {SERVERS} servers × 4MiB chunks, median of 3):");
    println!(
        "  single-server baseline: {:.0} MiB/s [{:.0}, {:.0}, {:.0}]",
        single / (1 << 20) as f64,
        singles[0] / (1 << 20) as f64,
        singles[1] / (1 << 20) as f64,
        singles[2] / (1 << 20) as f64
    );
    println!(
        "  pool concurrent capacity: {:.0} MiB/s [{:.0}, {:.0}, {:.0}]",
        pool_capacity / (1 << 20) as f64,
        pool_runs[0] / (1 << 20) as f64,
        pool_runs[1] / (1 << 20) as f64,
        pool_runs[2] / (1 << 20) as f64
    );
    println!(
        "  {CLIENTS}-client striped aggregate: {:.0} MiB/s [{:.0}, {:.0}, {:.0}]",
        aggregate / (1 << 20) as f64,
        aggregates[0] / (1 << 20) as f64,
        aggregates[1] / (1 << 20) as f64,
        aggregates[2] / (1 << 20) as f64
    );
    println!(
        "  ratio vs pool capacity: {:.1}% (gate >= 70%); informational vs 4x solo: {:.1}%",
        ratio * 100.0,
        solo_ratio * 100.0
    );

    assert!(
        ratio >= 0.70,
        "aggregate {aggregate:.0} B/s below 70% of pool capacity {pool_capacity:.0} B/s"
    );
    cluster.stop();
}
