#[cfg(test)]
mod tests {
    use amun_wal::WriteAheadLog;

    #[test]
    fn test_replay_certification_self_verifying() {
        let wal_path = "/tmp/amun_replay_cert_33.wal";
        let _ = WriteAheadLog::reset_for_testing(wal_path);

        let mut wal = WriteAheadLog::open(wal_path).unwrap();
        wal.append("QC", r#"{"block":"0x01"}"#).unwrap();
        wal.append("COMMIT", r#"{"block":"0x01"}"#).unwrap();
        wal.shutdown().unwrap();

        let wal = WriteAheadLog::open(wal_path).unwrap();
        let entries = wal.read_all().unwrap();
        assert_eq!(entries.len(), 2);
        assert!(WriteAheadLog::verify_chain_continuity(&entries).is_ok());

        let _ = WriteAheadLog::reset_for_testing(wal_path);
    }

    #[test]
    fn test_replay_idempotent() {
        let wal_path = "/tmp/amun_replay_idem_33.wal";
        let _ = WriteAheadLog::reset_for_testing(wal_path);

        let mut wal = WriteAheadLog::open(wal_path).unwrap();
        wal.append("QC", r#"{"block":"0x01"}"#).unwrap();
        wal.shutdown().unwrap();

        // Read twice - must be idempotent
        let wal1 = WriteAheadLog::open(wal_path).unwrap();
        let entries1 = wal1.read_all().unwrap();

        let wal2 = WriteAheadLog::open(wal_path).unwrap();
        let entries2 = wal2.read_all().unwrap();

        assert_eq!(entries1.len(), entries2.len());
        assert_eq!(entries1[0].sequence, entries2[0].sequence);

        let _ = WriteAheadLog::reset_for_testing(wal_path);
    }
}
