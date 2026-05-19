#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::evidence::*;
    use amun_kernel_types::*;

    #[test]
    fn test_evidence_type_creation() {
        let evidence = Evidence {
            evidence_type: EvidenceType::Equivocation,
            accused_validator: PublicKey::new([1u8; 48]),
            epoch: Epoch::new(1),
            round: Round::new(0),
            proof: EvidenceProof::Equivocation {
                position: EquivocationPosition {
                    epoch: Epoch::new(1),
                    round: Round::new(0),
                    step: 0,
                },
                first: SignedMessage {
                    validator: PublicKey::new([1u8; 48]),
                    message_hash: BlockHash::new([1u8; 32]),
                    signature: Signature::new([1u8; 96]),
                },
                second: SignedMessage {
                    validator: PublicKey::new([1u8; 48]),
                    message_hash: BlockHash::new([2u8; 32]),
                    signature: Signature::new([2u8; 96]),
                },
            },
        };

        let hash = evidence.compute_hash();
        assert_eq!(hash.as_bytes().len(), 32);
    }

    #[test]
    fn test_evidence_verify_same_hash_rejected() {
        let evidence = Evidence {
            evidence_type: EvidenceType::Equivocation,
            accused_validator: PublicKey::new([1u8; 48]),
            epoch: Epoch::new(1),
            round: Round::new(0),
            proof: EvidenceProof::Equivocation {
                position: EquivocationPosition {
                    epoch: Epoch::new(1),
                    round: Round::new(0),
                    step: 0,
                },
                first: SignedMessage {
                    validator: PublicKey::new([1u8; 48]),
                    message_hash: BlockHash::new([1u8; 32]),
                    signature: Signature::new([1u8; 96]),
                },
                second: SignedMessage {
                    validator: PublicKey::new([1u8; 48]),
                    message_hash: BlockHash::new([1u8; 32]),
                    signature: Signature::new([2u8; 96]),
                },
            },
        };

        assert!(evidence.verify().is_err());
    }
}
