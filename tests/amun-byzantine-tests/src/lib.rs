#[cfg(test)]
mod tests {
    use amun_crash_recovery::CrashRecovery;
    use amun_wal::WriteAheadLog;

    #[test]
    fn test_wal_integrity_under_byzantine_load() {
        let wal_path = "/tmp/amun_byzantine_test_33.wal";
        let _ = WriteAheadLog::reset_for_testing(wal_path);

        // Append entries
        {
            let mut wal = WriteAheadLog::open(wal_path).unwrap();
            for i in 1..=100 {
                wal.append("QC", &format!(r#"{{"block":"0x{:02x}"}}"#, i % 256))
                    .unwrap();
            }
            wal.shutdown().unwrap();
        }

        // Recover and verify
        let wal = WriteAheadLog::open(wal_path).unwrap();
        let integrity = wal.check_integrity().unwrap();
        assert!(
            integrity.is_clean,
            "WAL must be clean after normal operations"
        );

        let recovery = CrashRecovery::new(wal);
        assert!(recovery.verify_recovery().unwrap());

        let _ = WriteAheadLog::reset_for_testing(wal_path);
    }

    #[test]
    fn test_recovery_after_partial_crash() {
        let wal_path = "/tmp/amun_byzantine_crash_33.wal";
        let _ = WriteAheadLog::reset_for_testing(wal_path);

        // Write some entries then simulate crash (no shutdown)
        {
            let mut wal = WriteAheadLog::open(wal_path).unwrap();
            for i in 1..=50 {
                wal.append("QC", &format!(r#"{{"block":"0x{:02x}"}}"#, i % 256))
                    .unwrap();
            }
            // No shutdown - simulate crash
        }

        // Recover from unsealed segment
        let wal = WriteAheadLog::open(wal_path).unwrap();
        let entries = wal.read_all().unwrap();
        assert!(entries.len() >= 50, "Must recover at least 50 entries");

        let recovery = CrashRecovery::new(wal);
        assert!(recovery.verify_recovery().unwrap());

        let _ = WriteAheadLog::reset_for_testing(wal_path);
    }
}
