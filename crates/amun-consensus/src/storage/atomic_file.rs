use std::fs::{File, rename};
use std::io::Write;
use std::path::Path;

pub fn atomic_write(path: &str, bytes: &[u8]) -> std::io::Result<()> {
    let tmp = format!("{}.tmp", path);
    {
        let mut file = File::create(&tmp)?;
        file.write_all(bytes)?;
        file.sync_all()?;
    }
    rename(&tmp, path)?;
    if let Some(parent) = Path::new(path).parent() {
        let _ = File::open(parent)?.sync_all();
    }
    Ok(())
}
