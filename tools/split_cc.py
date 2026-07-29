#!/usr/bin/env python3
"""Build per-binary compile_commands.json from the MooseFS build tree.

For each <name>_SOURCES list in each Makefile.am, select compile db entries
whose canonical source file matches, preferring objects compiled with the
binary's own prefix (<name>-<file>.o) over convenience-library objects.
Libtool convenience libs (lib*_la-*.lo) are expanded when listed in LDADD.
"""
import json
import os
import re
import sys
import glob

BUILD = os.path.dirname(os.path.abspath(__file__))
SRC = "/tmp/moosefs-ref"

db = json.load(open(os.path.join(BUILD, "compile_commands.json")))
for e in db:
    e["canon"] = os.path.normpath(os.path.join(e["directory"], e["file"]))

# index: canonical path -> list of (objname, entry)
by_file = {}
for e in db:
    a = e["arguments"]
    obj = a[a.index("-o") + 1] if "-o" in a else ""
    by_file.setdefault(e["canon"], []).append((obj, e))


def parse_makefile_sources(path):
    """Return {target: [source files]} for all *_SOURCES in a Makefile.am."""
    text = open(path).read()
    # join line continuations
    text = re.sub(r"\\\n", " ", text)
    out = {}
    for m in re.finditer(r"^(\w+)_SOURCES\s*=\s*(.+)$", text, re.M):
        target = m.group(1)
        files = [f for f in m.group(2).split() if not f.startswith("$(")]
        out[target] = files
    # also capture LDADD libtool libs per target
    ldadd = {}
    for m in re.finditer(r"^(\w+)_(?:LDADD|LDFLAGS)\s*=\s*(.+)$", text, re.M):
        ldadd.setdefault(m.group(1), []).extend(
            re.findall(r"([\w/]+\.la)", m.group(2)))
    return out, ldadd


def lib_la_members(makefile_dir):
    """Map libfoo.la -> member prefix libfoo_la- using the Makefile.am _la_SOURCES."""
    out = {}
    text = open(os.path.join(makefile_dir, "Makefile.am")).read()
    text = re.sub(r"\\\n", " ", text)
    for m in re.finditer(r"^(\w+)_la_SOURCES\s*=\s*(.+)$", text, re.M):
        out[m.group(1) + ".la"] = m.group(2).split()
    return out


def pick_entries(target, files, makefile_dir):
    entries = []
    missing = []
    for f in files:
        if not f.endswith(".c"):
            continue
        canon = os.path.normpath(os.path.join(makefile_dir, f))
        cands = by_file.get(canon, [])
        if not cands:
            missing.append(canon)
            continue
        # prefer object prefixed with target name
        pref = [e for o, e in cands if re.search(rf"(^|/){re.escape(target)}-[^/]+\.o$", o)]
        unpref = [e for o, e in cands if re.search(rf"(^|/)[^/-]+\.o$", o)]
        chosen = pref or unpref or [cands[0][1]]
        entries.append(chosen[0])
    return entries, missing


def main():
    only = sys.argv[1:] or None
    for am in sorted(glob.glob(os.path.join(SRC, "*/Makefile.am"))):
        mdir = os.path.dirname(am)
        sources, ldadd = parse_makefile_sources(am)
        la_libs = lib_la_members(mdir)
        for target, files in sorted(sources.items()):
            if only and target not in only:
                continue
            all_files = list(files)
            for la in ldadd.get(target, []):
                base = os.path.basename(la)
                if base in la_libs:
                    all_files += la_libs[base]
            entries, missing = pick_entries(target, all_files, mdir)
            if missing:
                print(f"{target}: MISSING {missing}", file=sys.stderr)
            if not entries:
                continue
            out_entries = []
            for e in entries:
                e2 = {k: v for k, v in e.items() if k != "canon"}
                e2["file"] = e["canon"]
                out_entries.append(e2)
            out = os.path.join(BUILD, f"cc-{target}.json")
            json.dump(out_entries, open(out, "w"), indent=1)
            print(f"{target}: {len(entries)} TUs -> {out}")


if __name__ == "__main__":
    main()
