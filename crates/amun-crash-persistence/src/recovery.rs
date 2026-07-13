use amun_crash_recovery::CrashRecovery;
use amun_wal::WriteAheadLog;

pub fn recover_from_wal(wal_path: &str) -> Result<CrashRecovery, String> {
    let wal = WriteAheadLog::open(wal_path)?;
    Ok(CrashRecovery::new(wal))
}
