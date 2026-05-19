#[cfg(test)] mod tests { use crate::*;
    #[test] fn test_token_transfer() { let mut t = Token::new(1000); t.transfer(500).unwrap(); assert_eq!(t.balance, 500); }
    #[test] fn test_token_transfer_fail() { let mut t = Token::new(100); assert!(t.transfer(200).is_err()); }
    #[test] fn test_token_stake() { let mut t = Token::new(constants::MIN_STAKE_AMOUNT*2); t.stake(constants::MIN_STAKE_AMOUNT).unwrap(); assert!(t.staked > 0); }
    #[test] fn test_token_slash() { let mut t = Token::new(0); t.staked = 10000; t.slash(500).unwrap(); assert_eq!(t.staked, 9500); }
    #[test] fn test_fee() { let fm = FeeMarket::new(); assert!(fm.calculate_fee(10).unwrap() > 0); }
    #[test] fn test_reward() { let rd = RewardDistributor::new(); let (v, _) = rd.distribute(100, 50); assert!(v > 0); }
    #[test] fn test_treasury() { let mut t = Treasury::new(); t.deposit(100).unwrap(); t.withdraw(50).unwrap(); assert_eq!(t.balance, 50); }
}
