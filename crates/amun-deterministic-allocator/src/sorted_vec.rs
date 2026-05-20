use core::cmp::Ord;

const MAX_ENTRIES: usize = 1024;

#[derive(Debug, Clone)]
pub struct SortedVec<T: Ord + Clone> {
    entries: Vec<T>,
}

impl<T: Ord + Clone> SortedVec<T> {
    pub fn new() -> Self {
        Self {
            entries: Vec::with_capacity(MAX_ENTRIES),
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), &'static str> {
        if self.entries.len() >= MAX_ENTRIES {
            return Err("sorted vec capacity exceeded");
        }
        let pos = self.entries.binary_search(&value).unwrap_or_else(|e| e);
        self.entries.insert(pos, value);
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.entries.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.entries.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn as_slice(&self) -> &[T] {
        &self.entries
    }
}

impl<T: Ord + Clone> Default for SortedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}
