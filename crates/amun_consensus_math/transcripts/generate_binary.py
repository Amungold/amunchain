#!/usr/bin/env python3
import struct
import hashlib

SCALE = 1_000_000
MAX_FIXED = 10_000_000_000 * SCALE
MIN_FIXED = -MAX_FIXED

OP_SQRT = 1
OP_EXP = 2

def fixed_mul(a, b):
    return (a * b) // SCALE

def fixed_div(a, b):
    if b == 0:
        return 0
    return (a * SCALE) // b

def f_exp_py(x):
    if x <= -10 * SCALE:
        return 0
    if x >= 10 * SCALE:
        return MAX_FIXED
    
    x_float = x / SCALE
    result = 1.0
    term = 1.0
    
    for n in range(1, 20):
        term *= x_float / n
        result += term
    
    result_raw = int(round(result * SCALE))
    return max(MIN_FIXED, min(MAX_FIXED, result_raw))

def f_sqrt_py(x):
    if x <= 0:
        return 0
    y = (x + SCALE) // 2
    for _ in range(30):
        x_div_y = (x * SCALE) // y
        y_next = (y + x_div_y) // 2
        if abs(y_next - y) <= 1:
            return y_next
        y = y_next
    return y

def generate():
    test_cases = []
    
    # sqrt tests
    for inp in [10000, 250000, 500000, 1000000, 2000000, 4000000, 9000000, 16000000]:
        test_cases.append((OP_SQRT, inp, f_sqrt_py(inp)))
    
    # exp tests - use values within domain where no clipping occurs
    for inp in [-1000000, -500000, 0, 500000, 1000000, 2000000]:
        test_cases.append((OP_EXP, inp, f_exp_py(inp)))
    
    output_path = "crates/amun_consensus_math/transcripts/canonical.bin"
    hasher = hashlib.sha256()
    
    with open(output_path, 'wb') as f:
        f.write(struct.pack('>I', 2))
        
        for op, inp, out in test_cases:
            f.write(struct.pack('>B', op))
            f.write(struct.pack('>q', inp))
            f.write(struct.pack('>q', out))
            hasher.update(struct.pack('>q', inp))
            hasher.update(struct.pack('>q', out))
    
    hash_path = output_path.replace('.bin', '.hash')
    with open(hash_path, 'w') as f:
        f.write(hasher.hexdigest())
    
    print(f"Generated: {output_path}")
    print(f"Test cases: {len(test_cases)}")
    print(f"Hash: {hasher.hexdigest()}")

if __name__ == "__main__":
    generate()
