use std::fs;
use std::path::Path;

use amun_networking::crypto_identity::PeerKeyPair;

use crate::error::{io_err, NodeError};

pub fn load_or_create_keypair(key_file: &str) -> Result<PeerKeyPair, NodeError> {
    let path = Path::new(key_file);
    if path.exists() {
        let key_bytes = fs::read(path).map_err(|e| io_err(key_file, e))?;
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&key_bytes[..32]);
        tracing::info!(file = %key_file, "Loaded existing identity");
        Ok(PeerKeyPair::from_seed(seed))
    } else {
        let keypair = PeerKeyPair::generate();
        let seed = keypair.to_seed();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| io_err(parent.to_string_lossy().as_ref(), e))?;
        }
        fs::write(path, seed).map_err(|e| io_err(key_file, e))?;
        tracing::info!(file = %key_file, "Generated new identity and saved");
        Ok(keypair)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_generate_new_keypair() {
        let temp_dir = tempfile::tempdir().unwrap();
        let key_path = temp_dir.path().join("new_key.bin");
        let keypair = load_or_create_keypair(key_path.to_str().unwrap()).unwrap();
        let peer_id = keypair.peer_id();
        assert_eq!(peer_id.0.len(), 32);
        assert!(key_path.exists());
    }

    #[test]
    fn test_load_existing_keypair() {
        let temp_dir = tempfile::tempdir().unwrap();
        let key_path = temp_dir.path().join("existing_key.bin");
        let original = PeerKeyPair::generate();
        let seed = original.to_seed();
        let mut file = std::fs::File::create(&key_path).unwrap();
        file.write_all(&seed).unwrap();
        let loaded = load_or_create_keypair(key_path.to_str().unwrap()).unwrap();
        assert_eq!(original.to_seed(), loaded.to_seed());
    }

    #[test]
    fn test_keypair_determinism() {
        let temp_dir = tempfile::tempdir().unwrap();
        let key_path = temp_dir.path().join("deterministic_key.bin");
        let seed = [42u8; 32];
        let mut file = std::fs::File::create(&key_path).unwrap();
        file.write_all(&seed).unwrap();
        let keypair = load_or_create_keypair(key_path.to_str().unwrap()).unwrap();
        assert_eq!(keypair.to_seed(), seed);
    }

    #[test]
    fn test_creates_parent_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let key_path = temp_dir.path().join("subdir").join("nested_key.bin");
        assert!(!key_path.parent().unwrap().exists());
        let _keypair = load_or_create_keypair(key_path.to_str().unwrap()).unwrap();
        assert!(key_path.exists());
    }
}
