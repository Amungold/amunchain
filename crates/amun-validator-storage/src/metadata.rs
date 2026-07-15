use amun_validator_api::error::{PlatformError, PlatformResult, StorageError, StorageErrorCode};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Persistent metadata store — survives restarts.
/// Uses JSON on disk. Caches in memory for fast reads.
pub struct MetadataStore {
    path: PathBuf,
    data: Mutex<HashMap<String, serde_json::Value>>,
}

impl MetadataStore {
    pub fn new(dir: &Path) -> PlatformResult<Self> {
        fs::create_dir_all(dir).map_err(|e| {
            PlatformError::Storage(StorageError::new(
                StorageErrorCode::InitializationFailed,
                format!("Metadata dir: {}", e),
            ))
        })?;
        let file_path = dir.join("metadata.json");
        let data: HashMap<String, serde_json::Value> = if file_path.exists() {
            let content = fs::read_to_string(&file_path).unwrap_or_default();
            if content.is_empty() {
                HashMap::new()
            } else {
                serde_json::from_str(&content).unwrap_or_default()
            }
        } else {
            HashMap::new()
        };
        Ok(MetadataStore {
            path: dir.to_path_buf(),
            data: Mutex::new(data),
        })
    }

    fn save(&self) -> PlatformResult<()> {
        let data = self.data.lock().unwrap_or_else(|e| e.into_inner());
        let json = serde_json::to_string_pretty(&*data).unwrap_or_default();
        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(self.path.join("metadata.json"), &json).map_err(|e| {
            PlatformError::Storage(StorageError::new(
                StorageErrorCode::InitializationFailed,
                format!("Save: {}", e),
            ))
        })?;
        Ok(())
    }

    pub fn is_healthy(&self) -> bool {
        self.path.exists()
    }

    pub fn verify(&self) -> PlatformResult<()> {
        if !self.is_healthy() {
            return Err(PlatformError::Storage(StorageError::new(
                StorageErrorCode::DatabaseCorrupted,
                "Metadata dir missing".into(),
            )));
        }
        Ok(())
    }

    pub fn get_u64(&self, key: &str) -> PlatformResult<u64> {
        let data = self.data.lock().unwrap_or_else(|e| e.into_inner());
        Ok(data.get(key).and_then(|v| v.as_u64()).unwrap_or(0))
    }

    pub fn set_u64(&self, key: &str, value: u64) -> PlatformResult<()> {
        {
            let mut d = self.data.lock().unwrap_or_else(|e| e.into_inner());
            d.insert(key.to_string(), serde_json::json!(value));
        }
        self.save()
    }

    pub fn get_bytes32(&self, key: &str) -> PlatformResult<[u8; 32]> {
        let data = self.data.lock().unwrap_or_else(|e| e.into_inner());
        match data.get(key).and_then(|v| v.as_str()) {
            Some(hex_str) => {
                let bytes = hex::decode(hex_str).unwrap_or_default();
                let mut arr = [0u8; 32];
                let len = bytes.len().min(32);
                arr[..len].copy_from_slice(&bytes[..len]);
                Ok(arr)
            }
            None => Ok([0u8; 32]),
        }
    }

    pub fn set_bytes32(&self, key: &str, value: &[u8; 32]) -> PlatformResult<()> {
        let hex_str = hex::encode(value);
        {
            let mut d = self.data.lock().unwrap_or_else(|e| e.into_inner());
            d.insert(key.to_string(), serde_json::json!(hex_str));
        }
        self.save()
    }

    pub fn get_string(&self, key: &str) -> PlatformResult<String> {
        let data = self.data.lock().unwrap_or_else(|e| e.into_inner());
        Ok(data
            .get(key)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string())
    }

    pub fn set_string(&self, key: &str, value: &str) -> PlatformResult<()> {
        {
            let mut d = self.data.lock().unwrap_or_else(|e| e.into_inner());
            d.insert(key.to_string(), serde_json::json!(value));
        }
        self.save()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u64_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let s = MetadataStore::new(dir.path()).unwrap();
        s.set_u64("h", 42).unwrap();
        assert_eq!(s.get_u64("h").unwrap(), 42);
    }

    #[test]
    fn test_bytes32() {
        let dir = tempfile::tempdir().unwrap();
        let s = MetadataStore::new(dir.path()).unwrap();
        let r = [0xAA; 32];
        s.set_bytes32("r", &r).unwrap();
        assert_eq!(s.get_bytes32("r").unwrap(), r);
    }

    #[test]
    fn test_string() {
        let dir = tempfile::tempdir().unwrap();
        let s = MetadataStore::new(dir.path()).unwrap();
        s.set_string("c", "mainnet-1").unwrap();
        assert_eq!(s.get_string("c").unwrap(), "mainnet-1");
    }
}
