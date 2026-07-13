use amun_ntr::constants::*;
use amun_tokenomics::*;

#[test]
fn test_epoch_reward_calculation() {
    let total_supply = 8_000_000_000u64;

    let reward = EpochEconomics::compute_epoch_rewards(total_supply);

    let expected = ((total_supply as u128 * NTR_INITIAL_INFLATION_BPS as u128) / 10000u128) as u64;

    assert_eq!(reward, expected);
}

#[test]
fn test_distribution_splits() {
    let reward = 100_000_000u64;

    let (treasury, validators, ecosystem) = EpochEconomics::compute_distribution(reward);

    assert_eq!(
        treasury,
        reward * NTR_TREASURY_ALLOCATION_BPS as u64 / 10000
    );

    assert_eq!(
        validators,
        reward * NTR_STAKING_ALLOCATION_BPS as u64 / 10000
    );

    assert_eq!(
        ecosystem,
        reward * NTR_ECOSYSTEM_ALLOCATION_BPS as u64 / 10000
    );
}

#[test]
fn test_distribution_matches_configured_bps() {
    let total_bps = NTR_TREASURY_ALLOCATION_BPS as u64
        + NTR_STAKING_ALLOCATION_BPS as u64
        + NTR_ECOSYSTEM_ALLOCATION_BPS as u64;

    assert_eq!(total_bps, 3500);
}
