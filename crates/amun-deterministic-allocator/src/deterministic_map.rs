const MAX_ENTRIES: usize = 256;

#[derive(Debug, Clone)]
pub struct DeterministicMap<K: Ord + Clone, V: Clone> {
    entries: Vec<(K, V)>,
}

impl<K: Ord + Clone, V: Clone> DeterministicMap<K, V> {
    pub fn new() -> Self { Self { entries: Vec::with_capacity(MAX_ENTRIES) } }

    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, &'static str> {
        if let Some(pos) = self.entries.iter().position(|(k, _)| *k == key) {
            let old = self.entries[pos].1.clone();
            self.entries[pos] = (key, value);
            Ok(Some(old))
        } else {
            if self.entries.len() >= MAX_ENTRIES { return Err("map full"); }
            let pos = self.entries.binary_search_by(|(k, _)| k.cmp(&key)).unwrap_or_else(|e| e);
            self.entries.insert(pos, (key, value));
            Ok(None)
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries.binary_search_by(|(k, _)| k.cmp(key)).ok().map(|i| &self.entries[i].1)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.entries.binary_search_by(|(k, _)| k.cmp(key)).ok().map(|i| self.entries.remove(i).1)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.entries.binary_search_by(|(k, _)| k.cmp(key)).is_ok()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> + '_ {
        self.entries.iter().map(|(k, v)| (k, v))
    }

    pub fn len(&self) -> usize { self.entries.len() }
}

impl<K: Ord + Clone, V: Clone> Default for DeterministicMap<K, V> {
    fn default() -> Self { Self::new() }
}
