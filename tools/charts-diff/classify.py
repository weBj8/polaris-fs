"""Classify mismatches: 'tail' = PNGs whose decoded image rows are identical
and differ only past the image inside the deflated rawchart buffer (C
compresses an uninitialized malloc'd tail after re-init); anything else is
a real difference."""
import struct, sys, zlib
from cmp import recs

def raw(png):
    w, h = struct.unpack_from('>II', png, 16)
    o = 8; data = b''
    while o < len(png):
        l = struct.unpack_from('>I', png, o)[0]
        if png[o+4:o+8] == b'IDAT': data += png[o+8:o+8+l]
        o += 12 + l
    return w, h, zlib.decompress(data)

tail = real = 0
for x, y in zip(recs(sys.argv[1]), recs(sys.argv[2])):
    if x == y: continue
    if x[:4] == y[:4] == b'\x89PNG' and x[16:24] == y[16:24]:
        w, h, ra = raw(x); _, _, rb = raw(y)
        n = (w + 1) * h
        if ra[:n] == rb[:n] and len(ra) == len(rb):
            tail += 1; continue
    real += 1
print('tail-only', tail, 'real', real)
