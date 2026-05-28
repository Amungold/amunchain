"""
Compliance tests for constitutional math kernel.
Ensures all validators produce identical results.
"""

import sys
import json
sys.path.insert(0, '/root/projects/amunchain/amunchain')

from phase_80a.math.ops import (
    to_fixed, from_fixed, F_sqrt, F_exp, FIXED_E,
    SCALE, MAX_TOTAL_COUPLING
)

def load_canonical():
    with open('/root/projects/amunchain/amunchain/phase_80a/tests/snapshots/canonical_math.json', 'r') as f:
        return json.load(f)

def test_compliance():
    print("\n=== Constitutional Math Compliance ===\n")
    canonical = load_canonical()
    all_passed = True
    
    print("Testing sqrt compliance:")
    for v_str, expected in canonical['tests']['sqrt'].items():
        v = float(v_str)
        result = from_fixed(F_sqrt(to_fixed(v)))
        error = abs(result - expected)
        status = "PASS" if error < 0.0001 else "FAIL"
        if error >= 0.0001:
            all_passed = False
        print(f"  sqrt({v:4.2f}) = {result:.6f} (expected {expected:.6f}) [{status}]")
    
    print("\nTesting exp compliance:")
    for v_str, expected in canonical['tests']['exp'].items():
        v = float(v_str)
        result = from_fixed(F_exp(to_fixed(v)))
        error = abs(result - expected)
        status = "PASS" if error < 0.0001 else "FAIL"
        if error >= 0.0001:
            all_passed = False
        print(f"  exp({v:4.1f}) = {result:.6f} (expected {expected:.6f}) [{status}]")
    
    print("\nTesting constants:")
    print(f"  SCALE: {SCALE} (expected {canonical['tests']['constants']['SCALE']})")
    print(f"  FIXED_E: {from_fixed(FIXED_E):.6f} (expected {canonical['tests']['constants']['FIXED_E']})")
    print(f"  MAX_TOTAL_COUPLING: {from_fixed(MAX_TOTAL_COUPLING):.2f} (expected {canonical['tests']['constants']['MAX_TOTAL_COUPLING']})")
    
    print("\n" + "=" * 50)
    if all_passed:
        print("ALL COMPLIANCE TESTS PASSED")
    else:
        print("COMPLIANCE TESTS FAILED")
    print("=" * 50)
    
    return all_passed

if __name__ == "__main__":
    test_compliance()
