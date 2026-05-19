#[cfg(test)]
mod tests {
    use crate::*;
    use amun_kernel_types::PublicKey;

    #[test]
    fn test_keygen_deterministic() {
        let seed = [1u8; 32];
        let kp1 = KeyPair::generate_deterministic(&seed);
        let kp2 = KeyPair::generate_deterministic(&seed);
        assert_eq!(kp1.public.0, kp2.public.0);
        assert_eq!(kp1.secret.bytes, kp2.secret.bytes);
    }
    #[test]
    fn test_sign_and_verify() {
        let kp = KeyPair::generate_deterministic(&[2u8; 32]);
        let msg = b"test message";
        let sig = sign(msg, &kp.secret).unwrap();
        assert!(verify(msg, &sig, &kp.public).unwrap());
    }
    #[test]
    fn test_invalid_signature_rejected() {
        let kp = KeyPair::generate_deterministic(&[3u8; 32]);
        let msg = b"test message";
        let zero_sig = [0u8; BLS_SIGNATURE_SIZE];
        assert!(!verify(msg, &zero_sig, &kp.public).unwrap());
    }
    #[test]
    fn test_aggregate_signatures() {
        let sigs: [[u8; BLS_SIGNATURE_SIZE]; 2] = [[1u8; BLS_SIGNATURE_SIZE], [2u8; BLS_SIGNATURE_SIZE]];
        let agg = aggregate_signatures(&sigs).unwrap();
        assert_ne!(agg, [0u8; BLS_SIGNATURE_SIZE]);
    }
    #[test]
    fn test_aggregate_public_keys() {
        let pk1 = PublicKey::new([1u8; 48]);
        let pk2 = PublicKey::new([2u8; 48]);
        let agg = aggregate_public_keys(&[pk1, pk2]).unwrap();
        assert_ne!(agg.0, [0u8; 48]);
    }
    #[test]
    fn test_aggregate_empty_rejected() {
        let sigs: [[u8; BLS_SIGNATURE_SIZE]; 0] = [];
        assert!(aggregate_signatures(&sigs).is_err());
    }
}
