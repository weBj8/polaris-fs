#!/usr/bin/env python3
"""Post-process c2rust (edition 2024) output for stable Rust.

- drop #![feature(...)] lines (raw_ref_op/strict_provenance are stable,
  extern_types is not needed)
- replace extern-block opaque type declarations `pub type X;` with
  zero-variant enums (c2rust's pre-0.19 style), inserted after the
  leading inner-attribute block
"""
import re
import sys
import glob
import os


def attr_block_end(lines):
    """Return index after the leading run of inner-attribute lines."""
    i = 0
    depth = 0
    started = False
    while i < len(lines):
        line = lines[i]
        if not started:
            if line.startswith("#!["):
                started = True
                depth = line.count("(") - line.count(")")
                if depth <= 0 and line.rstrip().endswith("]"):
                    i += 1
                    break
            else:
                break
        else:
            depth += line.count("(") - line.count(")")
            if depth <= 0:
                i += 1
                break
        i += 1
    return i


def process(path):
    s = open(path).read()
    names = re.findall(r"^\s*pub type (\w+);\s*$", s, re.M)
    if not names and "#![feature(" not in s:
        return False
    s = re.sub(r"^\s*pub type \w+;\s*\n", "", s, flags=re.M)
    s = re.sub(r"^#!\[feature\([^)]*\)\]\n", "", s, flags=re.M)
    if names:
        enums = "".join(f"pub enum {n} {{}}\n" for n in dict.fromkeys(names))
        lines = s.splitlines(keepends=True)
        pos = attr_block_end(lines)
        lines.insert(pos, enums)
        s = "".join(lines)
    open(path, "w").write(s)
    return True


def main():
    root = sys.argv[1]
    n = 0
    for p in glob.glob(os.path.join(root, "**", "*.rs"), recursive=True):
        if process(p):
            n += 1
    print(f"{root}: {n} files patched")


if __name__ == "__main__":
    main()
