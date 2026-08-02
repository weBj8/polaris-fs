# Container deployment

PolarisFS publishes verified x86_64 Linux images to
`ghcr.io/webj8/polaris-fs`. Every successful `mfs-rust` CI run produces
`latest` and an immutable 12-character commit tag. Production deployments
should use a commit tag.

This guide uses:

- Docker Compose for master, metalogger, GUI, and one FUSE mount.
- Podman Quadlet for a chunkserver and optional FUSE mount on another host.
- Host networking, matching MooseFS default ports.

## Ports

| Port | Service |
| ---: | --- |
| 9419 | master ↔ metalogger |
| 9420 | master ↔ chunkserver |
| 9421 | master ↔ clients/mounts |
| 9422 | chunkserver data service |
| 9425 | GUI |

Allow these ports only on trusted cluster networks. Default generated exports
permit read-write access; edit `EXPORTS` in the master service before exposing
port 9421 beyond a private network.

## Prerequisites

- x86_64 Linux with Docker Compose or Podman 5+.
- `/dev/fuse` on hosts running `plfsmount`.
- A routable, non-loopback master address for chunkservers.
- Persistent directories for metadata, metalogger state, and chunks.

The image includes libfuse 3.18.2. Host libfuse version does not affect the
container.

Latest verified deployment: commit `782bb27`, image tag
`782bb278208d`, digest
`sha256:64ca97aad72cd845b97dc689baf8e599394aaf8f89975e7d58aa69f9249982f9`.
CI run for `782bb27` (run `30768186487`) passed gates, release build,
workspace tests, and cluster smoke before publishing this image. Production
update was plfsmount-only (`docker compose up -d --no-deps plfsmount`);
master, metalogger, and GUI were not restarted. A stale FUSE mount from the
previous container had to be unmounted on the host before the new container
could start (`fusermount3 -uz` + `umount -l`). Post-deploy concurrent
4-way read/write smoke passed (`DEPLOY_SMOKE_OK`) with a clean log scan.

Previous verified deployment: commit `45ff734`, image tag
`45ff7343310a`, digest
`sha256:18e10073afd52a9d3160c4e32404bc8f626f45cb2fee5d07a4d1178d1b451556`.
CI run for `45ff734` (run `30764567563`) passed gates, release build,
workspace tests, and cluster smoke before publishing this image. Production
update was plfsmount-only (`docker compose up -d --no-deps plfsmount`);
master, metalogger, and GUI were not restarted. Post-deploy concurrent
read/write smoke passed (`DEPLOY_SMOKE_OK`) with a clean log scan.

Previous verified deployment: commit `7e4742d`, image tag
`7e4742d21cb9`, digest
`sha256:3df8ce060e99eac1bd1505961765cc219c00f644a89e5b65721f5e414186c7eb`.
CI run `30742916264` passed gates, release build, workspace tests, and cluster
smoke before publishing this image.

## Single-host Compose

`docker/docker-compose.single.yml` runs every core role on one machine. Set the
master address to that machine's LAN IP because chunkserver rejects loopback:

```bash
export PLFS_MASTER_HOST=192.168.1.10
export PLFS_IMAGE=ghcr.io/webj8/polaris-fs:latest
export PLFS_DATA_DIR=/var/lib/polarisfs
export PLFS_CHUNK_DIR=/var/lib/polarisfs/chunks
export PLFS_MOUNT_DIR=/mnt/plfs

sudo install -d "$PLFS_DATA_DIR"/{master,metalogger,chunkserver,chunks,gui} "$PLFS_MOUNT_DIR"
mountpoint -q "$PLFS_MOUNT_DIR" || sudo mount --bind "$PLFS_MOUNT_DIR" "$PLFS_MOUNT_DIR"
sudo mount --make-rshared "$PLFS_MOUNT_DIR"
sudo -E docker compose -f docker/docker-compose.single.yml up -d
sudo -E docker compose -f docker/docker-compose.single.yml ps
findmnt "$PLFS_MOUNT_DIR"
```

Use an immutable commit tag instead of `latest` for persistent deployments.
GUI listens on port 9425.

## Metadata tier with Docker Compose

Choose an immutable image tag from the
[GHCR package](https://github.com/weBj8/polaris-fs/pkgs/container/polaris-fs):

```bash
export PLFS_TAG=b1134a2f3957
export MASTER_IP=10.10.0.1

sudo install -d /root/docker/plfs /var/lib/polarisfs/{master,metalogger,gui} /mnt/plfs
sudo mountpoint -q /mnt/plfs || sudo mount --bind /mnt/plfs /mnt/plfs
sudo mount --make-rshared /mnt/plfs
sudo cp docker/docker-compose.yml /root/docker/plfs/

sudo tee /root/docker/plfs/.env >/dev/null <<EOF
PLFS_IMAGE=ghcr.io/webj8/polaris-fs:${PLFS_TAG}
PLFS_MASTER_HOST=${MASTER_IP}
PLFS_DATA_DIR=/var/lib/polarisfs
PLFS_MOUNT_DIR=/mnt/plfs
EOF

cd /root/docker/plfs
sudo docker compose pull
sudo docker compose up -d
sudo docker compose ps
findmnt /mnt/plfs
```

`PLFS_DATA_DIR` contains metadata and metalogger state. Keep it outside the
Compose directory so replacing deployment files cannot delete cluster data.

Check startup:

```bash
sudo docker logs --tail 100 plfsmaster
sudo docker logs --tail 100 plfsmetalogger
sudo docker logs --tail 100 plfsmount
```

Healthy master logs include listeners on ports 9419, 9420, and 9421.

## Chunkserver with Podman Quadlet

On the chunkserver host, install the repository templates under a dedicated
Quadlet directory:

```bash
export PLFS_TAG=b1134a2f3957
export MASTER_IP=10.10.0.1

install -d /etc/containers/systemd/plfs /var/lib/plfschunkserver /mnt/hdd/plfs /mnt/plfs
install -m 0644 docker/plfschunkserver.container /etc/containers/systemd/plfs/
install -m 0644 docker/plfsmount-nas.container /etc/containers/systemd/plfs/

sed -i \
  -e "s|ghcr.io/webj8/polaris-fs:latest|ghcr.io/webj8/polaris-fs:${PLFS_TAG}|" \
  -e "s|MASTER_HOST=192.168.1.1|MASTER_HOST=${MASTER_IP}|" \
  /etc/containers/systemd/plfs/*.container

mountpoint -q /mnt/plfs || mount --bind /mnt/plfs /mnt/plfs
mount --make-rshared /mnt/plfs
podman pull "ghcr.io/webj8/polaris-fs:${PLFS_TAG}"
systemctl daemon-reload
systemctl start plfschunkserver.service
systemctl start plfsmount.service
```

Quadlet units are generated units; do not run `systemctl enable` on them. Their
`[Install]` sections are processed by the Quadlet generator.

Verify:

```bash
systemctl status plfschunkserver.service plfsmount.service
podman logs --tail 100 plfschunkserver
findmnt /mnt/plfs

test_file=/mnt/plfs/.deployment-check
printf 'PolarisFS OK\n' >"$test_file"
sync
grep -q 'PolarisFS OK' "$test_file" && rm "$test_file"
```

Healthy chunkserver logs include `connected to Master` and either a completed
scan or `valid .chunkdb found`.

## Safe upgrades

Pulling a new image does not replace running containers. Upgrade in this order
to avoid storing stale client locks in metadata:

1. Snapshot master and metalogger data directories.
2. Pull the new immutable image tag on every host.
3. Stop all FUSE mounts.
4. Stop metaloggers and GUI, then stop master cleanly.
5. Stop chunkservers.
6. Update image tags in Compose and Quadlet files.
7. Start master and verify strict metadata loading.
8. Start metaloggers and chunkservers; confirm registration.
9. Start mounts and perform a cross-host checksum test.

Example metadata backup:

```bash
stamp=$(date +%Y%m%d-%H%M%S)
sudo cp -a --reflink=auto /var/lib/polarisfs/master \
  "/var/lib/polarisfs/master.backup-${stamp}"
sudo cp -a --reflink=auto /var/lib/polarisfs/metalogger \
  "/var/lib/polarisfs/metalogger.backup-${stamp}"
```

Never use `docker compose down -v`: it can remove named volumes. Never move or
delete chunk directories during a configuration-directory rename.

## Metadata recovery warning

If master reports `lock on closed file`, stop restart loops and preserve the
entire metadata and metalogger directories before recovery. `plfsmaster -i`
drops inconsistent metadata records and must only be tested against a snapshot.
A repaired snapshot is acceptable only after it can stop cleanly and reload
without `-i`. Do not run `-a` or `-i` directly against the only metadata copy.

## Rollback

Keep the previous immutable image tag and deployment files. Stop mounts first,
restore the previous tag, then restart in the safe-upgrade order. Data formats
remain MooseFS 4.59.2 compatible, but metadata backups are still mandatory
before every upgrade.
