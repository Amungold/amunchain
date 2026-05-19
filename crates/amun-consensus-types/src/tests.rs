#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::*;
    use amun_codec::CanonicalDecode;
    use amun_codec::CanonicalEncode;
    use amun_kernel_types::*;

    #[test]
    fn test_phase_discriminants_frozen() {
        assert_eq!(ConsensusPhase::Proposal as u8, 0x01);
        assert_eq!(ConsensusPhase::PrepareVote as u8, 0x02);
        assert_eq!(ConsensusPhase::PreCommitVote as u8, 0x03);
        assert_eq!(ConsensusPhase::CommitVote as u8, 0x04);
        assert_eq!(ConsensusPhase::TimeoutVote as u8, 0x05);
        assert_eq!(ConsensusPhase::TimeoutCertificate as u8, 0x06);
    }

    #[test]
    fn test_phase_is_vote() {
        assert!(!ConsensusPhase::Proposal.is_vote());
        assert!(ConsensusPhase::PrepareVote.is_vote());
        assert!(ConsensusPhase::PreCommitVote.is_vote());
        assert!(ConsensusPhase::CommitVote.is_vote());
        assert!(ConsensusPhase::TimeoutVote.is_vote());
        assert!(!ConsensusPhase::TimeoutCertificate.is_vote());
    }

    #[test]
    fn test_round_encoding() {
        let r = ConsensusRound::new(42);
        let mut buf = [0u8; 8];
        r.encode(&mut buf).unwrap();
        let decoded = ConsensusRound::decode_exact(&buf).unwrap();
        assert_eq!(decoded, r);
    }

    #[test]
    fn test_bitmap_set_get() {
        let mut bm = SignerBitmap::new();
        bm.set(0).unwrap();
        assert!(bm.get(0));
    }

    #[test]
    fn test_vote_wire_size_frozen() {
        assert_eq!(Vote::MAX_ENCODED_SIZE, 83);
    }

    #[test]
    fn test_qc_wire_size_frozen() {
        assert_eq!(QuorumCertificate::MAX_ENCODED_SIZE, 177);
    }

    #[test]
    fn test_vote_new_rejects_proposal() {
        assert!(Vote::new(
            ConsensusPhase::Proposal,
            Epoch::new(1),
            ConsensusRound::new(0),
            BlockHash::new([0u8; 32]),
            ValidatorIndex::new(0),
            ValidatorId::new([0u8; 32]),
        )
        .is_err());
    }

    #[test]
    fn test_qc_decode_rejects_proposal() {
        let mut buf = [0u8; 177];
        buf[0] = 0x01;
        assert!(QuorumCertificate::decode(&buf).is_err());
    }

    #[test]
    fn test_message_roundtrip() {
        let msg = ConsensusMessage::new(
            1,
            ChainId(1),
            Epoch::new(1),
            ConsensusPhase::PrepareVote,
            PublicHash32::new([0x42u8; 32]),
            b"hello",
        )
        .unwrap();
        let mut buf = [0u8; 2048];
        let len = msg.encode(&mut buf).unwrap();
        let decoded = ConsensusMessage::decode_exact(&buf[..len]).unwrap();
        assert_eq!(decoded.phase, ConsensusPhase::PrepareVote);
        assert_eq!(&decoded.payload[..], b"hello");
    }
}
