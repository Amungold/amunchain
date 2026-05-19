// Canonical sort order tests.
// Verifies stable sorting with deterministic byte-ordered comparison.
// All comparisons are HOMOGENEOUS (same type) by constitutional design.

#[cfg(test)]
mod sort_order {
    use amun_codec::canonical_sort::*;

    #[test]
    fn test_u64_sort_order() {
        let mut values = [3u64, 1, 4, 1, 5, 9, 2, 6];
        canonical_sort(&mut values).unwrap();
        assert_eq!(values, [1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_sort_is_stable_for_equal_values() {
        let mut values = [0u64, 0, 0];
        let original = values;
        canonical_sort(&mut values).unwrap();
        assert_eq!(values, original);
    }

    #[test]
    fn test_compare_is_total_order() {
        let ord = compare_by_canonical_bytes(&1u64, &2u64).unwrap();
        assert_eq!(ord, core::cmp::Ordering::Less);

        let ord = compare_by_canonical_bytes(&2u64, &1u64).unwrap();
        assert_eq!(ord, core::cmp::Ordering::Greater);

        let ord = compare_by_canonical_bytes(&42u64, &42u64).unwrap();
        assert_eq!(ord, core::cmp::Ordering::Equal);
    }

    #[test]
    fn test_sort_empty_slice() {
        let mut values: [u64; 0] = [];
        canonical_sort(&mut values).unwrap();
    }

    #[test]
    fn test_sort_single_element() {
        let mut values = [42u64];
        canonical_sort(&mut values).unwrap();
        assert_eq!(values, [42u64]);
    }

    #[test]
    fn test_sort_already_sorted() {
        let mut values = [1u64, 2, 3, 4, 5];
        let original = values;
        canonical_sort(&mut values).unwrap();
        assert_eq!(values, original);
    }

    #[test]
    fn test_sort_reverse_sorted() {
        let mut values = [5u64, 4, 3, 2, 1];
        canonical_sort(&mut values).unwrap();
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }
}
