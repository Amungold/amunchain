import sys
from .test_full_chain_integrity import test_full_chain_determinism
from .test_suffocation import test_suffocation, test_necromancy
from .test_storage import test_storage_integrity, test_foreign_keys
from .test_consensus import test_semantic_gate, test_validator

def run_all():
    print("=" * 60)
    print("PHASE 80A - Complete Test Suite")
    print("=" * 60)
    
    tests = [
        ("Full Chain Determinism", test_full_chain_determinism),
        ("Suffocation Detection", test_suffocation),
        ("Necromancy Prevention", test_necromancy),
        ("Storage Integrity", test_storage_integrity),
        ("Foreign Keys", test_foreign_keys),
        ("Semantic Gate", test_semantic_gate),
        ("Validator", test_validator)
    ]
    
    results = []
    for name, test_func in tests:
        print(f"\n--- {name} ---")
        try:
            result = test_func()
            results.append((name, result))
        except Exception as e:
            print(f"ERROR: {e}")
            import traceback
            traceback.print_exc()
            results.append((name, False))
    
    print("\n" + "=" * 60)
    print("SUMMARY")
    print("=" * 60)
    
    all_passed = True
    for name, passed in results:
        status = "PASSED" if passed else "FAILED"
        print(f"  {status}: {name}")
        if not passed:
            all_passed = False
    
    return all_passed

if __name__ == "__main__":
    success = run_all()
    sys.exit(0 if success else 1)
