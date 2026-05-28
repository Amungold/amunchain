use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

pub struct LruCache<K, V> {
    map: HashMap<K, V>,
    order: VecDeque<K>,
    capacity: usize,
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self { map: HashMap::new(), order: VecDeque::new(), capacity }
    }
    
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(val) = self.map.get(key) {
            self.order.retain(|k| k != key);
            self.order.push_back(key.clone());
            return Some(val);
        }
        None
    }
    
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        if let Some(old) = self.map.insert(key.clone(), value) {
            self.order.retain(|k| k != &key);
            self.order.push_back(key);
            return Some(old);
        }
        if self.map.len() > self.capacity {
            if let Some(oldest) = self.order.pop_front() {
                self.map.remove(&oldest);
            }
        }
        self.order.push_back(key);
        None
    }
}
