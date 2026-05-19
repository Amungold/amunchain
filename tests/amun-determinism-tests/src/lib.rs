#[cfg(test)]
mod tests {
    use amun_deterministic_allocator::DeterministicMap;

    #[test]
    fn test_deterministic_map_iteration_is_canonical() {
        let mut map = DeterministicMap::new();
        map.insert(5u64, "five").unwrap();
        map.insert(1u64, "one").unwrap();
        map.insert(3u64, "three").unwrap();
        map.insert(2u64, "two").unwrap();
        map.insert(4u64, "four").unwrap();

        let keys: Vec<u64> = map.iter().map(|(k, _)| *k).collect();
        assert_eq!(keys, vec![1, 2, 3, 4, 5], "Iteration must be canonical sorted order");
    }

    #[test]
    fn test_deterministic_map_remove() {
        let mut map = DeterministicMap::new();
        map.insert(1u64, "one").unwrap();
        map.insert(2u64, "two").unwrap();
        map.insert(3u64, "three").unwrap();

        let removed = map.remove(&2u64);
        assert_eq!(removed, Some("two"));

        let keys: Vec<u64> = map.iter().map(|(k, _)| *k).collect();
        assert_eq!(keys, vec![1, 3], "After removal, ordering must remain canonical");

        assert!(!map.contains_key(&2u64));
        assert!(map.contains_key(&1u64));
        assert!(map.contains_key(&3u64));
    }

    #[test]
    fn test_deterministic_map_overwrite() {
        let mut map = DeterministicMap::new();
        map.insert(1u64, "original").unwrap();
        let old = map.insert(1u64, "updated").unwrap();
        assert_eq!(old, Some("original"));

        let value = map.get(&1u64);
        assert_eq!(value, Some(&"updated"));
        assert_eq!(map.len(), 1);
    }
}
