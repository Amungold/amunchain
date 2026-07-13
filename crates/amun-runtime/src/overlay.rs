use amun_failure::AmunResult;
use heapless::Vec;

pub struct OverlayEntry {
    pub key: Vec<u8, 32>,
    pub value: Option<Vec<u8, 32>>,
}

pub struct OverlayState {
    entries: Vec<OverlayEntry, 64>,
}

impl OverlayState {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn set(&mut self, key: Vec<u8, 32>, value: Vec<u8, 32>) -> AmunResult<()> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.key == key) {
            entry.value = Some(value);
        } else {
            self.entries
                .push(OverlayEntry {
                    key,
                    value: Some(value),
                })
                .map_err(|_| {
                    amun_failure::FailureContext::new(
                        amun_failure::ConstitutionalFault::CapacityExceeded,
                        0x0010,
                        0x0001,
                    )
                })?;
        }
        Ok(())
    }

    pub fn delete(&mut self, key: Vec<u8, 32>) -> AmunResult<()> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.key == key) {
            entry.value = None;
        } else {
            self.entries
                .push(OverlayEntry { key, value: None })
                .map_err(|_| {
                    amun_failure::FailureContext::new(
                        amun_failure::ConstitutionalFault::CapacityExceeded,
                        0x0010,
                        0x0002,
                    )
                })?;
        }
        Ok(())
    }

    pub fn get(&self, key: &[u8; 32]) -> Option<&Vec<u8, 32>> {
        self.entries
            .iter()
            .find(|e| e.key.as_slice() == key)
            .and_then(|e| e.value.as_ref())
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for OverlayState {
    fn default() -> Self {
        Self::new()
    }
}
