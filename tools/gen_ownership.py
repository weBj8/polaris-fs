#!/usr/bin/env python3
"""Generate docs/facts/OWNERSHIP.tsv — every `static mut` in the tree,
pre-classified by type shape (LIFETIMES.tsv analogue, verified-claims VC-02).

Columns: crate · file · line · item · transpiled_type · class · safe_type · evidence

Classes (methodology.md 1b):
  STATIC   — scalar/flags; safe_type Atomic* (relaxed unless evidence otherwise)
  ONCE     — init-once-then-read (pointer set at startup); safe_type OnceLock/…
  BUF      — fixed buffer/struct; safe_type Mutex<…> or thread_local
  UNKNOWN  — pointer/aggregate needing per-site analysis; consumer = owning phase

Evidence column: `auto:type-shape` (this script) or later `verified:<commit>`
when a phase promotes a row with real analysis. Rows are never deleted;
reclassification appends a correction row with `supersedes:` in evidence.
"""
import re, subprocess, sys, os

os.chdir(os.path.dirname(os.path.abspath(__file__)) + "/..")

DECL = re.compile(r'\bstatic\s+mut\s+([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(.+?)\s*(?:=[^=].*)?;?\s*$')
SCALARS = {
    'uint8_t':'AtomicU8','uint16_t':'AtomicU16','uint32_t':'AtomicU32','uint64_t':'AtomicU64',
    'int8_t':'AtomicI8','int16_t':'AtomicI16','int32_t':'AtomicI32','int64_t':'AtomicI64',
    'libc::c_int':'AtomicI32','libc::c_uint':'AtomicU32','libc::c_long':'AtomicI64',
    'libc::c_ulong':'AtomicU64','libc::c_char':'AtomicI8','libc::c_schar':'AtomicI8',
    'libc::c_uchar':'AtomicU8','libc::c_short':'AtomicI16','libc::c_ushort':'AtomicU16',
    'libc::c_longlong':'AtomicI64','libc::c_ulonglong':'AtomicU64',
    'c_int':'AtomicI32','c_uint':'AtomicU32','c_long':'AtomicI64','c_ulong':'AtomicU64',
    'c_char':'AtomicI8','c_uchar':'AtomicU8','c_short':'AtomicI16','c_ushort':'AtomicU16',
    'u8':'AtomicU8','u16':'AtomicU16','u32':'AtomicU32','u64':'AtomicU64','usize':'AtomicUsize',
    'i8':'AtomicI8','i16':'AtomicI16','i32':'AtomicI32','i64':'AtomicI64','isize':'AtomicIsize',
    'bool':'AtomicBool','sig_atomic_t':'AtomicI32','mode_t':'AtomicU32','pid_t':'AtomicI32',
    'time_t':'AtomicI64','size_t':'AtomicUsize','ssize_t':'AtomicIsize',
}

def classify(ty):
    t = ty.strip()
    if t in SCALARS:
        return ('STATIC', SCALARS[t])
    if t.startswith('*mut ') or t.startswith('*const '):
        return ('UNKNOWN', 'Option<NonNull<_>> + per-site analysis')
    if t.startswith('['):
        return ('BUF', 'Mutex<[…; N]> or thread_local')
    return ('BUF', 'Mutex<%s> or thread_local' % t)

rows = []
out = subprocess.run(['grep','-rEn',r'static mut [A-Za-z_0-9]+ *:','--include=*.rs','.'],
                     capture_output=True, text=True).stdout
unparsed = []
for line in out.splitlines():
    path, lno, text = line.split(':', 2)
    crate = path.split('/')[1]
    m = DECL.search(text)
    if not m:
        unparsed.append(line); continue
    name, ty = m.group(1), m.group(2)
    cls, safe = classify(ty)
    rows.append((crate, path[2:], lno, name, ty, cls, safe, 'auto:type-shape'))

os.makedirs('docs/facts', exist_ok=True)
with open('docs/facts/OWNERSHIP.tsv','w') as f:
    f.write('# crate\tfile\tline\titem\ttranspiled_type\tclass\tsafe_type\tevidence\n')
    for r in sorted(rows):
        f.write('\t'.join(r)+'\n')

from collections import Counter
c = Counter(r[5] for r in rows)
cr = Counter(r[0] for r in rows)
print(f'rows={len(rows)} classes={dict(c)}')
print('per-crate:', dict(cr))
if unparsed:
    print(f'UNPARSED={len(unparsed)}', *unparsed[:5], sep='\n')
