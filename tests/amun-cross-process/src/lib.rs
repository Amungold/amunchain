#[cfg(test)]
mod tests {
    use amun_crash_recovery::CrashRecovery;
    use amun_wal::WriteAheadLog;

    #[test]
    fn test_cross_process_wal_recovery() {
        let wal_path = "/tmp/amun_cross_proc_33.wal";
        let _ = WriteAheadLog::reset_for_testing(wal_path);

        // Process A: Write entries
        {
            let mut wal = WriteAheadLog::open(wal_path).unwrap();
            for i in 1..=20 {
                wal.append("QC", &format!(r#"{{"block":"0x{:02x}"}}"#, i % 256))
                    .unwrap();
            }
            wal.shutdown().unwrap();
        }

        // Process B: Recover and verify
        let wal = WriteAheadLog::open(wal_path).unwrap();
        let recovery = CrashRecovery::new(wal);
        assert!(recovery.verify_recovery().unwrap());

        let _ = WriteAheadLog::reset_for_testing(wal_path);
    }

    #[test]
    fn test_cross_process_crash_recovery() {
        let wal_path = "/tmp/amun_cross_crash_33.wal";
        let _ = WriteAheadLog::reset_for_testing(wal_path);

        // Write and crash (no shutdown)
        {
            let mut wal = WriteAheadLog::open(wal_path).unwrap();
            for i in 1..=30 {
                wal.append("QC", &format!(r#"{{"block":"0x{:02x}"}}"#, i % 256))
                    .unwrap();
            }
        }

        // Recover
        let wal = WriteAheadLog::open(wal_path).unwrap();
        let entries = wal.read_all().unwrap();
        assert!(entries.len() >= 30);

        let _ = WriteAheadLog::reset_for_testing(wal_path);
    }
}
