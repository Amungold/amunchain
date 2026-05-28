"""
Overflow safety tests for fixed-point arithmetic.
Ensures operations stay within safe bounds.
"""

import sys
sys.path.insert(0, '/root/projects/amunchain/amunchain')

from phase_80a.math.ops import (
    SCALE, to_fixed, from_fixed, to_fixed_int,
    F_add, F_sub, F_mul, F_div, F_exp, F_sqrt,
    ONE, HALF
)

def test_overflow_safety():
    print("\n=== Overflow Safety Tests ===\n")
    
    max_safe = 2**62 - 1
    large_value = to_fixed_int(10**9)
    
    # Test multiplication
    mul_result = F_mul(large_value, large_value)
    if mul_result < max_safe:
        print(f"  Multiplication: {from_fixed(mul_result):.2e} < 2^62 [PASS]")
    else:
        print(f"  Multiplication overflow risk: {mul_result} [FAIL]")
    
    # Test addition of large values
    add_result = F_add(large_value, large_value)
    if add_result < max_safe:
        print(f"  Addition: {from_fixed(add_result):.2e} < 2^62 [PASS]")
    else:
        print(f"  Addition overflow risk: {add_result} [FAIL]")
    
    # Test exponential on large input
    exp_result = F_exp(to_fixed_int(10))
    if exp_result <= ONE:
        print(f"  Exponential (10): {from_fixed(exp_result)} [SATURATED]")
    else:
        print(f"  Exponential (10): {from_fixed(exp_result):.2e} [PASS]")
    
    # Test repeated squaring (entropy storm simulation)
    val = to_fixed_int(2)
    for i in range(10):
        val = F_mul(val, val)
        if val > max_safe:
            print(f"  Entropy storm: overflow at iteration {i+1} [WARNING]")
            break
    else:
        print(f"  Entropy storm: 10 iterations safe [PASS]")
    
    # Test boundary values
    zero = F_add(ONE, F_neg(ONE))
    if zero == 0:
        print(f"  Boundary zero: {zero} [PASS]")
    else:
        print(f"  Boundary zero: {zero} [FAIL]")
    
    print("\n=== Overflow Tests Complete ===\n")

if __name__ == "__main__":
    test_overflow_safety()
