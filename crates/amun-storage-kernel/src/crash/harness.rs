use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;

pub struct CrashSimulator {
    // TempDir is held for RAII cleanup - directory is deleted on drop
    #[allow(dead_code)]
    dir: tempfile::TempDir,
    wal_path: PathBuf,
}

impl CrashSimulator {
    pub fn new() -> Self {
        let dir = tempdir().unwrap();
        let wal_path = dir.path().join("test.wal");
        Self { dir, wal_path }
    }

    pub fn write_partial(&self, data: &[u8]) -> std::io::Result<()> {
        let f = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&self.wal_path)?;
        let mut file = f;
        file.write_all(data)?;
        file.sync_all()?;
        Ok(())
    }

    pub fn truncate_wal(&self, keep_bytes: usize) -> std::io::Result<()> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&self.wal_path)?;
        file.set_len(keep_bytes as u64)?;
        file.sync_all()?;
        Ok(())
    }

    pub fn read_wal(&self) -> Vec<u8> {
        std::fs::read(&self.wal_path).unwrap_or_default()
    }
}

impl Default for CrashSimulator {
    fn default() -> Self {
        Self::new()
    }
}
