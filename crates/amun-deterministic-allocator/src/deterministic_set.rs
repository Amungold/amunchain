use core::cmp::Ord;

const MAX_ENTRIES: usize = 256;

#[derive(Debug, Clone)]
pub struct DeterministicSet<T: Ord + Clone> {
    entries: Vec<T>,
}

impl<T: Ord + Clone> DeterministicSet<T> {
    pub fn new() -> Self {
        Self {
            entries: Vec::with_capacity(MAX_ENTRIES),
        }
    }

    pub fn insert(&mut self, value: T) -> Result<bool, &'static str> {
        if let Err(pos) = self.entries.binary_search(&value) {
            if self.entries.len() >= MAX_ENTRIES {
                return Err("set capacity exceeded");
            }
            self.entries.insert(pos, value);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.entries.binary_search(value).is_ok()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<T: Ord + Clone> Default for DeterministicSet<T> {
    fn default() -> Self {
        Self::new()
    }
}
