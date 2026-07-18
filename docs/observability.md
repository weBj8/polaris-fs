# Observability

P12 adds two operator-facing surfaces without changing the chunkserver wire
protocol:

- a Prometheus scrape endpoint on `porfs chunkserver`
- a read-only `porfsadm status` CLI backed by the existing `Stats` RPC

## Structured tracing

`porfs` and `porfsadm` initialize `tracing` on startup. Logs go to stderr and are
disabled by default below `warn`; set `RUST_LOG` when you want structured
command/activity logs.

```bash
RUST_LOG=info ./target/release/porfs chunkserver \
    --device /var/lib/porfs/chunk0.img --size 16TiB \
    --listen 0.0.0.0:9100 \
    --metrics-listen 127.0.0.1:9900
```

The code uses bounded structured fields such as `command`, `device`, `listen`,
and `addr`; it never emits unbounded metric labels derived from client input.

## Prometheus endpoint

Enable the exporter with `--metrics-listen`:

```bash
./target/release/porfs chunkserver \
    --device /var/lib/porfs/chunk0.img --size 16TiB \
    --listen 0.0.0.0:9100 \
    --metrics-listen 127.0.0.1:9900

curl -s http://127.0.0.1:9900/metrics | grep '^porfs_'
```

### Exported metrics

All labels are bounded enums.

| Metric | Type | Labels | Meaning |
|---|---|---|---|
| `porfs_rpc_requests_total` | counter | `op` | Total RPC requests handled by the chunkserver. |
| `porfs_rpc_request_errors_total` | counter | `op`, `code` | Failed RPCs by request kind and protocol error code. |
| `porfs_rpc_request_duration_seconds` | histogram | `op` | End-to-end server-side latency per request, including chain replication when applicable. |
| `porfs_rpc_write_bytes_total` | counter | — | User payload bytes accepted by successful `WriteExtent` calls. |
| `porfs_rpc_read_bytes_total` | counter | — | User payload bytes returned by successful `ReadExtent` calls. |
| `porfs_chunkserver_store_capacity_bytes` | gauge | — | Total device size. |
| `porfs_chunkserver_store_tail_bytes` | gauge | — | Current append-log tail (allocated bytes high-water mark). |
| `porfs_chunkserver_store_live_bytes` | gauge | — | Bytes still referenced by live extents. |
| `porfs_chunkserver_store_extent_count` | gauge | — | Live extent count. |
| `porfs_chunkserver_store_confirmed_watermark` | gauge | — | Highest extent id confirmed durable by `Sync`. |

`op` is one of `hello`, `write_extent`, `read_extent`, `tombstone`, `sync`, or
`stats`. `code` is one of the finite protocol error enums (`bad_request`,
`not_found`, `corrupt`, `store_full`, `version_mismatch`, `internal`,
`replica_unavailable`).

## `porfsadm`

Use `porfsadm` when you want a quick point-in-time status check without scraping
Prometheus:

```bash
./target/release/porfsadm status --addr 127.0.0.1:9100
```

Example output:

```text
chunkserver:    127.0.0.1:9100
device_size:    17592186044416 (16.0 TiB)
tail:           4194304 (4.0 MiB)
free_bytes:     17592181850112 (16.0 TiB)
live_bytes:     1048576 (1.0 MiB)
extent_count:   1
confirmed_id:   1
```

`tail` tracks append-log allocation pressure; `live_bytes` tracks logically live
data after tombstones; `confirmed_id` is the durability watermark that survives
crashes after `Sync`.

## Reading the signals

- High `porfs_rpc_request_duration_seconds` for `read_extent` with low
  `live_bytes`/stable `tail` usually points at network or per-request overhead.
- Rising `tail_bytes` with flat `live_bytes` indicates tombstoned garbage
  accumulation rather than true data growth.
- Rising latency together with `tail_bytes` approaching `capacity_bytes` points
  at store-pressure/device-full risk.
- `porfsadm status` is the fastest way to confirm the current durable
  `confirmed_id` watermark during incident response.
