#!/usr/bin/env python3
"""Remove `unnecessary unsafe` blocks flagged by rustc (unused_unsafe lint).
The JSON span covers only the `unsafe` keyword; we delete it, then remove
the matching closing brace of the block (brace counting) and dedent the
inner lines one level. Run: tools/strip_unused_unsafe.py <crate_dir>
Then: cargo fmt (formatting safety net) + rebuild.
"""
import json, subprocess, sys, collections

crate = sys.argv[1]
proc = subprocess.run(
    ["cargo", "build", "--release", "--message-format=json"],
    cwd=crate, capture_output=True, text=True)
spans = []
for line in proc.stdout.splitlines():
    try:
        msg = json.loads(line)
    except json.JSONDecodeError:
        continue
    if msg.get("reason") != "compiler-message":
        continue
    m = msg["message"]
    if m.get("code") and m["code"].get("code") == "unused_unsafe":
        for sp in m["spans"]:
            if sp["is_primary"]:
                spans.append((sp["file_name"], sp["byte_start"], sp["byte_end"]))

def find_close(data, open_pos):
    depth = 0
    i = open_pos
    while i < len(data):
        c = data[i:i+1]
        if c == b"{":
            depth += 1
        elif c == b"}":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return -1

byfile = collections.defaultdict(list)
for f, s, e in spans:
    byfile[f].append((s, e))

removed = 0
for f, spans_f in byfile.items():
    path = f if f.startswith(crate) else f"{crate}/{f}"
    data = open(path, "rb").read()
    edits = []
    for s, e in spans_f:
        if data[s:e] != b"unsafe":
            print(f"SKIP unexpected span text: {f}:{s} {data[s:e]!r}")
            continue
        # find the '{' after the keyword
        j = data.index(b"{", e)
        close = find_close(data, j)
        if close < 0:
            print(f"SKIP no close brace: {f}:{s}")
            continue
        edits.append((s, e, j, close))
    for s, e, j, close in sorted(edits, reverse=True):
        inner = data[j+1:close]
        # dedent inner lines by up to 4 spaces
        lines = inner.split(b"\n")
        lines = [l[4:] if l.startswith(b"    ") else l for l in lines]
        data = data[:s] + b"\n".join(lines).strip(b"\n") + data[close+1:]
    open(path, "wb").write(data)
    removed += len(edits)
print(f"{crate}: removed {removed} unnecessary unsafe wrappers")
