use blake3::Hasher;
use std::fs;
use std::path::Path;

const MAX_WAL_ENTRIES: usize = 1_000_000;

pub struct NonceStore {
    pub nonces: Vec<([u8; 32], u64)>,
    pub wal_path: Option<String>,
    pub persistent: bool,
}

impl NonceStore {
    pub fn new() -> Self {
        Self {
            nonces: Vec::new(),
            wal_path: None,
            persistent: false,
        }
    }

    pub fn with_wal(path: &str) -> Self {
        let mut store = Self {
            nonces: Vec::new(),
            wal_path: Some(path.to_string()),
            persistent: true,
        };
        store.load_wal();
        store.nonces.sort_by_key(|(k, _)| *k);
        store
    }

    pub fn check_and_update(&mut self, account: &[u8; 32], nonce: u64) -> Result<(), &'static str> {
        let pos = self.nonces.binary_search_by_key(account, |(k, _)| *k);

        match pos {
            Ok(idx) => {
                let (_, last_nonce) = self.nonces[idx];
                if nonce <= last_nonce {
                    return Err("nonce too low");
                }
                self.nonces[idx].1 = nonce;
            }
            Err(idx) => {
                if nonce == 0 {
                    return Err("nonce too low");
                }
                self.nonces.insert(idx, (*account, nonce));
            }
        }

        if self.nonces.len() > MAX_WAL_ENTRIES {
            return Err("nonce store capacity exceeded");
        }

        if self.persistent {
            self.append_wal(account, nonce);
        }
        Ok(())
    }

    fn append_wal(&mut self, account: &[u8; 32], nonce: u64) {
        if let Some(ref path) = self.wal_path {
            let entry = format!("{}:{}\n", hex::encode(account), nonce);
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
            {
                let _ = std::io::Write::write_all(&mut file, entry.as_bytes());
                let _ = file.sync_all();
            }
        }
    }

    fn load_wal(&mut self) {
        if let Some(ref path) = self.wal_path {
            if Path::new(path).exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    let mut count = 0;
                    for line in content.lines() {
                        if count >= MAX_WAL_ENTRIES {
                            break;
                        }
                        if let Some((hex_key, nonce_str)) = line.split_once(':') {
                            if let Ok(key) = hex::decode(hex_key) {
                                if key.len() == 32 {
                                    if let Ok(nonce) = nonce_str.parse::<u64>() {
                                        let mut k = [0u8; 32];
                                        k.copy_from_slice(&key);
                                        match self.nonces.binary_search_by_key(&k, |(ak, _)| *ak) {
                                            Ok(idx) => {
                                                if nonce > self.nonces[idx].1 {
                                                    self.nonces[idx].1 = nonce;
                                                }
                                            }
                                            Err(idx) => {
                                                self.nonces.insert(idx, (k, nonce));
                                                count += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn count(&self) -> usize {
        self.nonces.len()
    }

    pub fn merkle_root(&self) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(b"NONCE_STORE_V4");
        for (k, v) in &self.nonces {
            hasher.update(k);
            hasher.update(&v.to_le_bytes());
        }
        let h = hasher.finalize();
        let mut out = [0u8; 32];
        out.copy_from_slice(&h.as_bytes()[..32]);
        out
    }
}

impl Default for NonceStore {
    fn default() -> Self {
        Self::new()
    }
}
