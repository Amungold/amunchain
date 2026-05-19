#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_topic_roundtrip() {
        for topic in &[Topic::Blocks, Topic::Votes, Topic::QuorumCert, Topic::Transactions] {
            let b = topic.as_byte();
            let decoded = Topic::from_byte(b).unwrap();
            assert_eq!(*topic, decoded);
        }
    }
    #[test]
    fn test_dedup_detects_duplicate() {
        let mut dedup = DedupCache::new();
        let hash = [1u8; 32];
        assert!(!dedup.is_duplicate(&hash));
        dedup.mark_seen(hash);
        assert!(dedup.is_duplicate(&hash));
    }
    #[test]
    fn test_fanout_selects_correct_count() {
        let f = Fanout::new();
        assert_eq!(f.select_peers(10, true), 6);
        assert_eq!(f.select_peers(3, true), 2);
        assert_eq!(f.select_peers(0, true), 0);
    }
    #[test]
    fn test_broadcaster_rejects_duplicate() {
        let mut b = Broadcaster::new();
        let payload = b"test block";
        let count1 = b.broadcast(Topic::Blocks, payload, 10).unwrap();
        assert!(count1 > 0);
        let count2 = b.broadcast(Topic::Blocks, payload, 10).unwrap();
        assert_eq!(count2, 0);
    }
    #[test]
    fn test_receiver_counts_unique() {
        let mut r = Receiver::new();
        assert!(r.receive(Topic::Votes, b"vote1").unwrap());
        assert_eq!(r.received_count(), 1);
        assert!(!r.receive(Topic::Votes, b"vote1").unwrap());
        assert_eq!(r.received_count(), 1);
        assert!(r.receive(Topic::Votes, b"vote2").unwrap());
        assert_eq!(r.received_count(), 2);
    }
    #[test]
    fn test_retry_backoff() {
        let mut rm = RetryManager::new();
        assert!(rm.should_retry());
        assert_eq!(rm.backoff_ms(), 2000);
        rm.retry();
        assert_eq!(rm.backoff_ms(), 4000);
        rm.retry();
        assert_eq!(rm.backoff_ms(), 8000);
    }
}
