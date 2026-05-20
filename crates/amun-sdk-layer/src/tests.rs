#[cfg(test)]
    use crate::*;

    #[test]
    fn test_token_api_create() { let result = TokenApi::create_account(1_000_000); assert!(result.success); }
    #[test]
    fn test_token_api_transfer() { let mut token = TokenApi::create_account(1_000_000).data.expect("test invariant"); let result = TokenApi::transfer(&mut token, 100_000); assert!(result.success); }
    #[test]
    fn test_staking_api_register() { let mut api = StakingApi::new(); let pk = amun_kernel_types::PublicKey::new([1u8; 48]); let result = api.register_validator(pk, 1_000_000); assert!(result.success); }
    #[test]
    fn test_governance_api_create_proposal() { let mut api = GovernanceApi::new(); let proposer = amun_kernel_types::PublicHash32::new([1u8; 32]); let result = api.create_proposal(proposer, amun_governance::proposal::ProposalType::Text, 1000); assert!(result.success); }
    #[test]
    fn test_charity_api_donate() { let mut api = CharityApi::new(); let recipient = amun_kernel_types::PublicHash32::new([1u8; 32]); let result = api.donate(recipient, 100); assert!(result.success); }
    #[test]
    fn test_sandbox_simulation() { let mut sandbox = Sandbox::new(); let a0 = sandbox.create_account(1_000_000).data.expect("test invariant"); let a1 = sandbox.create_account(500_000).data.expect("test invariant"); let result = sandbox.simulate_transfer(a0, a1, 100_000); assert!(result.success); }
    #[test]
    fn test_transaction_builder_transfer() { let sender = amun_kernel_types::PublicKey::new([1u8; 48]); let recipient = amun_kernel_types::PublicHash32::new([2u8; 32]); let result = TransactionBuilder::build_transfer(42, 0, sender, recipient, 100, 1000); assert!(result.success); }
