#[cfg(test)] mod tests { use crate::*;
    #[test] fn test_transfer_tax() { let mut t = NtrToken::new(1_000_000); let tax = t.transfer(100_000).expect("test invariant"); assert!(tax > 0); assert!(t.burned > 0); }
    #[test] fn test_stake() { let mut t = NtrToken::new(constants::NTR_MIN_STAKE_AMOUNT*2); t.stake(constants::NTR_MIN_STAKE_AMOUNT).expect("test invariant"); assert_eq!(t.staked, constants::NTR_MIN_STAKE_AMOUNT); }
    #[test] fn test_slash() { let mut t = NtrToken::new(0); t.staked = 10000; let s = t.slash(500); assert_eq!(s, 500); assert_eq!(t.staked, 9500); }
}
