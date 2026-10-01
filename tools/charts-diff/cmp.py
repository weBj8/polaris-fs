import struct, sys
def recs(p):
    b = open(p, 'rb').read(); o = 0; out = []
    while o < len(b):
        l = struct.unpack_from('<I', b, o)[0]; out.append(b[o+4:o+4+l]); o += 4 + l
    return out
a, b = recs(sys.argv[1]), recs(sys.argv[2])
print('records', len(a), len(b))
bad = 0
for i, (x, y) in enumerate(zip(a, b)):
    if x != y:
        bad += 1
        if bad <= 8:
            d = next((k for k in range(min(len(x), len(y))) if x[k] != y[k]), min(len(x), len(y)))
            print('rec', i, 'len', len(x), len(y), 'first diff at', d, x[max(0,d-8):d+8].hex(), y[max(0,d-8):d+8].hex())
print('mismatched records', bad)
