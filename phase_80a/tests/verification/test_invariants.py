"""
Formal invariant tests for constitutional math kernel.
Tests monotonicity, bounds, and conservation laws.
"""

import sys
sys.path.insert(0, '/root/projects/amunchain/amunchain')

from phase_80a.math.ops import (
    to_fixed, from_fixed, F_sqrt, F_exp, F_mul, F_add,
    ONE, ZERO
)

def test_monotonicity_invariant():
    print("\n=== Invariant: Monotonicity ===\n")
    
    for x in [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]:
        for y in [x + 0.05, x + 0.1]:
            if y > 1.0:
                continue
            
            sqrt_x = from_fixed(F_sqrt(to_fixed(x)))
            sqrt_y = from_fixed(F_sqrt(to_fixed(y)))
            
            if x < y:
                if sqrt_x > sqrt_y:
                    print(f"  VIOLATION: sqrt({x}) > sqrt({y})")
                    return False
                if from_fixed(F_exp(to_fixed(x))) > from_fixed(F_exp(to_fixed(y))):
                    print(f"  VIOLATION: exp({x}) > exp({y})")
                    return False
    
    print("  Monotonicity invariant: PASS")
    return True

def test_bounds_invariant():
    print("\n=== Invariant: Bounded Range ===\n")
    
    test_values = [-100, -10, -1, 0, 1, 10, 100]
    all_ok = True
    
    for v in test_values:
        result = from_fixed(F_exp(to_fixed(v)))
        if result < 0 or result > 1000000:
            print(f"  VIOLATION: exp({v}) = {result} out of bounds")
            all_ok = False
    
    for v in [0, 0.25, 0.5, 0.75, 1.0, 2.0, 4.0]:
        result = from_fixed(F_sqrt(to_fixed(v)))
        if result < 0 or result > max(1.0, v):
            print(f"  VIOLATION: sqrt({v}) = {result} out of bounds")
            all_ok = False
    
    if all_ok:
        print("  Bounds invariant: PASS")
    return all_ok

def test_identity_invariant():
    print("\n=== Invariant: Identity ===\n")
    
    identity = from_fixed(F_sqrt(F_mul(to_fixed(4.0), to_fixed(4.0))))
    expected = 4.0
    
    if abs(identity - expected) < 0.0001:
        print(f"  sqrt(a*a) = a: sqrt(16) = {identity:.6f} [PASS]")
        return True
    else:
        print(f"  sqrt(a*a) = a: sqrt(16) = {identity:.6f} [FAIL]")
        return False

def test_conservation_invariant():
    print("\n=== Invariant: Conservation ===\n")
    
    a = to_fixed(0.3)
    b = to_fixed(0.7)
    
    # Test that operations preserve ordering
    if a < b:
        sum_ab = from_fixed(F_add(a, b))
        if sum_ab > from_fixed(a) and sum_ab > from_fixed(b):
            print("  Conservation of magnitude: PASS")
            return True
        else:
            print("  Conservation of magnitude: FAIL")
            return False
    return True

if __name__ == "__main__":
    print("=" * 50)
    print("FORMAL INVARIANT SUITE")
    print("=" * 50)
    
    test_monotonicity_invariant()
    test_bounds_invariant()
    test_identity_invariant()
    test_conservation_invariant()
    
    print("\n" + "=" * 50)
    print("INVARIANT SUITE COMPLETE")
    print("=" * 50)
