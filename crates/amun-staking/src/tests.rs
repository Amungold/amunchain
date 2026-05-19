#[cfg(test)] mod tests { use crate::*; use amun_kernel_types::PublicKey;
    #[test] fn test_register() { let mut r = ValidatorRegistry::new(); r.register(PublicKey::new([1u8;48]), 100).unwrap(); assert_eq!(r.active_count, 1); }
    #[test] fn test_slash() { let mut r = ValidatorRegistry::new(); let pk = PublicKey::new([1u8;48]); r.register(pk, 10000).unwrap(); let rules = SlashingConditions::new(); assert!(r.slash(&pk, &rules).unwrap() > 0); }
    #[test] fn test_delegate() { let mut d = DelegationManager::new(); let mut s = 100; d.delegate(&mut s, 50).unwrap(); assert_eq!(s, 150); }
}
