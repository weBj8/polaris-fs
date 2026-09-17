#!/usr/bin/env python3
"""plfs-snapshot.py — MooseFS/polaris-fs snapshot helper.

Talks to the mounted filesystem's master proxy (the mfsmount "tool socket")
exactly like the C `mfsmakesnapshot`/`mfsrmsnapshot` tools (see
mfsclient/masterproxy.c and mfsclient/tools_snapshots.c):

  * discover the proxy from the mount's virtual `.masterinfo` file
    (walking up from the given path, like C `open_master_conn`),
  * register as a tool client (CLTOMA_FUSE_REGISTER, REGISTER_TOOLS),
  * create snapshots (CLTOMA_FUSE_SNAPSHOT),
  * delete old snapshots (CLTOMA_FUSE_SNAPSHOT + SNAPSHOT_MODE_DELETE).

Intended use: a systemd .timer runs this every N hours so each run saves a
dated snapshot directory and prunes all but the --keep most recent ones.

Wire protocol (all big-endian):
  request : [cmd:4][len:4][msgid:4][payload...]   # len == 4 + len(payload)
  reply   : [acmd:4][len:4][msgid:4][payload...]  # ANTOAN_NOP (cmd 0) skipped

  CLTOMA_FUSE_REGISTER (400), payload must be 73 bytes: 64-byte ACL magic
    + REGISTER_TOOLS (4). Reply MATOCL_FUSE_REGISTER (401): [status:1].

  CLTOMA_FUSE_SNAPSHOT (468) -> MATOCL_FUSE_SNAPSHOT (469):
    payload: [srcinode:4][dstinode:4][namelen:1][name]
             [uid:4][gidcount:4][gid*:4]         # gidcount = 1+extra-groups
             [smode:1][umask:2]
    reply:   [msgid:4][status:1]

Gotcha that bit us in production (2026-09-17): the master's fast delete
(SNAPSHOT_MODE_DELETE, fsnodes_remove_snapshot_test) only accepts a tree
whose nodes ALL carry EATTR_SNAPSHOT, and returns MFS_ERROR_EPERM (1)
otherwise.  The dated directories this tool creates are plain FUSE mkdirs
(wrapper "<date>/"), NOT snapshot nodes — only "<date>/<src-base>" is.
So the fast delete must be aimed at the snapshot itself, and anything the
fast path refuses (modified snapshots etc.) falls back to a plain
recursive rm, which the master serves through the normal unlink/rmdir path.
"""

import argparse
import os
import shutil
import socket
import struct
import sys
import time

PROTO_BASE = 0
CLTOMA_FUSE_REGISTER = PROTO_BASE + 400
MATOCL_FUSE_REGISTER = PROTO_BASE + 401
CLTOMA_FUSE_SNAPSHOT = PROTO_BASE + 468
MATOCL_FUSE_SNAPSHOT = PROTO_BASE + 469
ANTOAN_NOP = 0

# Must match the mount's masterproxy.c FUSE_REGISTER_BLOB_ACL.
FUSE_REGISTER_BLOB_ACL = (
    b"DjI1GAQDULI5d2YjA26ypc3ovkhjvhciTQVx3CS4nYgtBoUcsljiVpsErJENHaw0"
)
REGISTER_TOOLS = 4

SNAPSHOT_MODE_CAN_OVERWRITE     = 0x01
SNAPSHOT_MODE_CPLIKE_ATTR       = 0x02
SNAPSHOT_MODE_FORCE_REMOVAL     = 0x04
SNAPSHOT_MODE_PRESERVE_HARDLINK = 0x08
SNAPSHOT_MODE_DELETE            = 0x80

MFS_STATUS_OK = 0
MASTERINFO_NAME = ".masterinfo"


class SnapshotError(Exception):
    pass


class Proxy:
    def __init__(self, host, port, timeout=30):
        self.sock = socket.create_connection((host, port), timeout)
        self.sock.settimeout(timeout)

    def close(self):
        try:
            self.sock.close()
        finally:
            self.sock = None

    def _recv_exact(self, n):
        buf = b""
        while len(buf) < n:
            chunk = self.sock.recv(n - len(buf))
            if not chunk:
                raise SnapshotError("connection closed by proxy")
            buf += chunk
        return buf

    def _recv(self):
        """Read one answer, skipping proxy keep-alive ANTOAN_NOP packets."""
        while True:
            acmd, alen = struct.unpack(">II", self._recv_exact(8))
            if acmd != ANTOAN_NOP:
                return acmd, self._recv_exact(alen)

    def _send(self, cmd, body):
        self.sock.sendall(struct.pack(">II", cmd, len(body)) + body)

    # -- high level -----------------------------------------------------
    def register(self):
        # Proxy requires exactly 73 bytes: 64-byte ACL magic, 1 byte
        # REGISTER_TOOLS, then 4-byte cuid + 2-byte major + 1 mid + 1 min
        # (C tools_common.c master_register). The proxy only validates the
        # magic and the REGISTER_TOOLS byte; the trailing fields are
        # carried/ignored, so zero the cuid + version.
        body = FUSE_REGISTER_BLOB_ACL + bytes([REGISTER_TOOLS]) + bytes(8)
        self._send(CLTOMA_FUSE_REGISTER, body)
        acmd, reply = self._recv()
        if acmd != MATOCL_FUSE_REGISTER or len(reply) < 1:
            raise SnapshotError("register: bad reply (acmd=%d len=%d)" %
                                (acmd, len(reply)))
        if reply[0] != MFS_STATUS_OK:
            raise SnapshotError("register: status %d" % reply[0])

    def snapshot(self, src_inode, dst_dir_inode, name, smode,
                 uid, gid, groups, umask):
        """One snapshot request; returns master status byte (0 == OK)."""
        if isinstance(name, str):
            name = name.encode()
        if not 0 < len(name) <= 255:
            raise SnapshotError("invalid name length")

        body = struct.pack(">IIB", src_inode, dst_dir_inode, len(name)) + name

        # gid set: primary gid is counted via addmaingroup unless it already
        # appears in the supplementary groups (mirrors C tools_snapshots.c).
        body += struct.pack(">I", uid)
        addmaingroup = 1 if gid not in groups else 0
        gidcount = addmaingroup + len(groups)
        body += struct.pack(">I", gidcount)
        if addmaingroup:
            body += struct.pack(">I", gid)
        for g in groups:
            body += struct.pack(">I", g)
        body += struct.pack(">BH", smode, umask)

        # The proxy expects the request body to start with the msgid.
        self._send(CLTOMA_FUSE_SNAPSHOT, struct.pack(">I", 0) + body)
        acmd, reply = self._recv()
        if acmd != MATOCL_FUSE_SNAPSHOT:
            raise SnapshotError("snapshot: wrong answer type (%d)" % acmd)
        if len(reply) < 5:
            raise SnapshotError("snapshot: short reply (%d bytes)" % len(reply))
        return reply[4]


def read_masterinfo(start_path):
    """Walk up from start_path to find the mount's `.masterinfo` file.
    Returns (ip, port). Layout: [ip:4][port:2][cuid:4][ver:4][pid:8]."""
    p = os.path.realpath(start_path)
    if not os.path.lexists(p):
        raise SnapshotError("%s: realpath error" % start_path)
    while True:
        cand = os.path.join(p, MASTERINFO_NAME)
        if os.path.lexists(cand):
            with open(cand, "rb") as f:
                data = f.read()
            if len(data) < 6:
                raise SnapshotError("%s: short masterinfo (%d bytes)" %
                                    (cand, len(data)))
            ip = "%d.%d.%d.%d" % struct.unpack("4B", data[0:4])
            port = struct.unpack(">H", data[4:6])[0]
            return ip, port
        parent = os.path.dirname(p)
        if parent == p:
            raise SnapshotError(
                "%s: not a MooseFS object (no .masterinfo found)" % start_path)
        p = parent


def get_inode(path):
    return os.lstat(os.path.realpath(path)).st_ino


def current_ctx():
    return os.getuid(), os.getgid(), list(os.getgroups())


def read_umask():
    m = os.umask(0)
    os.umask(m)
    return m


def make_snapshot(proxy, src, dst_dir, smode):
    """Port of C `make_snapshot` for the simple file/dir-into-directory case:
    create snapshot named `<src basename>` under `dst_dir`."""
    uid, gid, groups = current_ctx()
    umask = read_umask()
    src_inode = get_inode(src)
    dst_inode = get_inode(dst_dir) if dst_dir else 1
    name = os.path.basename(os.path.realpath(src))
    status = proxy.snapshot(src_inode, dst_inode, name, smode,
                            uid, gid, groups, umask)
    if status != MFS_STATUS_OK:
        raise SnapshotError("snapshot %s failed (status %d)" % (src, status))
    return os.path.join(dst_dir, name)


def remove_snapshot(proxy, path):
    """Delete a snapshot via SNAPSHOT_MODE_DELETE (fast master-side rm).

    `path` must be the snapshot itself (a tree whose nodes all carry
    EATTR_SNAPSHOT), e.g. "<date>/ssd-back" — NOT the plain-FUSE wrapper
    "<date>", which the master refuses with MFS_ERROR_EPERM.
    """
    dirn = os.path.dirname(path) or "."
    base = os.path.basename(path.rstrip("/"))
    if not base:
        raise SnapshotError("can't remove %s" % path)
    uid, gid, groups = current_ctx()
    umask = read_umask()
    status = proxy.snapshot(0, get_inode(dirn), base, SNAPSHOT_MODE_DELETE,
                            uid, gid, groups, umask)
    if status != MFS_STATUS_OK:
        raise SnapshotError("remove %s: status %d error" % (path, status))


def prune(proxy, snap_root, keep):
    """Delete all but the `keep` most recent snapshot directories.

    Each dated directory is a plain FUSE-made wrapper around the real
    snapshot(s) ("<date>/<src-base>").  Fast-delete each snapshot child,
    fall back to a plain recursive rm for anything the master's fast path
    refuses (modified snapshots, non-snapshot content), then rmdir the
    empty wrapper.
    """
    if not os.path.isdir(snap_root):
        return
    entries = []
    for name in os.listdir(snap_root):
        p = os.path.join(snap_root, name)
        if os.path.isdir(p):
            entries.append((name, p))
    entries.sort(key=lambda e: e[0])
    for name, p in entries[:-keep] if keep else entries:
        for child in os.listdir(p):
            cp = os.path.join(p, child)
            try:
                remove_snapshot(proxy, cp)
            except SnapshotError:
                # not a pristine snapshot tree (EATTR_SNAPSHOT missing
                # somewhere, e.g. content modified inside the snapshot):
                # the normal unlink/rmdir path handles it fine.
                shutil.rmtree(cp, ignore_errors=True)
        try:
            os.rmdir(p)
        except OSError:
            pass


def cmd_snapshot(args):
    host, port = read_masterinfo(args.source[0])
    proxy = Proxy(host, port)
    try:
        proxy.register()
        os.makedirs(args.snap_dir, exist_ok=True)
        snap_root = os.path.realpath(args.snap_dir)
        newdir = os.path.join(snap_root, time.strftime("%Y%m%d-%H%M%S"))
        os.mkdir(newdir)
        for src in args.source:
            make_snapshot(proxy, src, newdir, 0)
        prune(proxy, snap_root, args.keep)
        print("created %s" % newdir)
    finally:
        proxy.close()


def cmd_rm(args):
    host, port = read_masterinfo(args.path[0])
    proxy = Proxy(host, port)
    try:
        proxy.register()
        for p in args.path:
            remove_snapshot(proxy, p)
    finally:
        proxy.close()


def build_parser():
    ap = argparse.ArgumentParser(
        description="MooseFS/polaris-fs snapshot helper (talks to the "
                    "mounted filesystem's master proxy).")
    sub = ap.add_subparsers(dest="cmd", required=True)

    snap = sub.add_parser("snapshot",
                          help="create dated snapshots and prune old ones")
    snap.add_argument("--snap-dir", required=True,
                      help="base directory (inside the mount) to hold snapshots")
    snap.add_argument("--keep", type=int, default=20,
                      help="keep this many most-recent snapshots (default 20)")
    snap.add_argument("source", nargs="+", help="files/dirs to snapshot")
    snap.set_defaults(fn=cmd_snapshot)

    rm = sub.add_parser("rm", help="delete snapshots (quick master-side rm)")
    rm.add_argument("path", nargs="+", help="snapshot paths to delete")
    rm.set_defaults(fn=cmd_rm)

    return ap


def main(argv=None):
    args = build_parser().parse_args(argv)
    try:
        args.fn(args)
    except SnapshotError as e:
        print("plfs-snapshot: %s" % e, file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
