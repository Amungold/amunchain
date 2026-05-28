"""
Monotonicity tests for fixed-point functions.
Ensures sqrt and exp preserve ordering.
"""

import sys
sys.path.insert(0, '/root/projects/amunchain/amunchain')

from phase_80a.math.ops import (
    to_fixed, from_fixed, F_sqrt, F_exp,
    ZERO, ONE, HALF
)

def test_sqrt_monotonicity():
    print("\n=== Square Root Monotonicity ===\n")
    
    values = [0.001, 0.01, 0.1, 0.25, 0.5, 0.75, 1.0, 2.0, 4.0, 10.0, 100.0]
    sqrt_values = []
    monotonic = True
    
    for v in values:
        result = from_fixed(F_sqrt(to_fixed(v)))
        sqrt_values.append(result)
        print(f"  sqrt({v:6.3f}) = {result:.6f}")
    
    for i in range(1, len(sqrt_values)):
        if sqrt_values[i] < sqrt_values[i-1]:
            print(f"  MONOTONICITY VIOLATION at {values[i]}")
            monotonic = False
    
    if monotonic:
        print("\n  Square root is monotonic [PASS]")
    else:
        print("\n  Square root is NOT monotonic [FAIL]")
    
    return monotonic

def test_exp_monotonicity():
    print("\n=== Exponential Monotonicity ===\n")
    
    values = [-5.0, -2.0, -1.0, -0.5, 0.0, 0.5, 1.0, 2.0, 5.0]
    exp_values = []
    monotonic = True
    
    for v in values:
        result = from_fixed(F_exp(to_fixed(v)))
        exp_values.append(result)
        print(f"  exp({v:4.1f}) = {result:.6f}")
    
    for i in range(1, len(exp_values)):
        if exp_values[i] < exp_values[i-1]:
            print(f"  MONOTONICITY VIOLATION at exp({values[i]})")
            monotonic = False
    
    if monotonic:
        print("\n  Exponential is monotonic [PASS]")
    else:
        print("\n  Exponential is NOT monotonic [FAIL]")
    
    return monotonic

def test_ordering_preservation():
    print("\n=== Ordering Preservation ===\n")
    
    a = to_fixed(0.3)
    b = to_fixed(0.6)
    
    if a < b:
        print("  a < b [TRUE]")
    else:
        print("  a < b [FALSE]")
    
    sqrt_a = F_sqrt(a)
    sqrt_b = F_sqrt(b)
    
    if sqrt_a < sqrt_b:
        print("  sqrt(a) < sqrt(b) [TRUE - monotonic preserved]")
    else:
        print("  sqrt(a) < sqrt(b) [FALSE - monotonic broken]")
    
    exp_a = F_exp(a)
    exp_b = F_exp(b)
    
    if exp_a < exp_b:
        print("  exp(a) < exp(b) [TRUE - monotonic preserved]")
    else:
        print("  exp(a) < exp(b) [FALSE - monotonic broken]")

if __name__ == "__main__":
    test_sqrt_monotonicity()
    test_exp_monotonicity()
    test_ordering_preservation()
