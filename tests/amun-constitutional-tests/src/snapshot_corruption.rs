use amun_state_root::ConstitutionalSnapshot;
use amun_state_root::ContinuityChain;
use amun_state_root::ReplayCertificate;
use amun_state_root::ReplayEquivalenceProof;
use amun_state_root::ReplayTranscript;

fn make_snapshot(epoch: u64, parent_hash: [u8; 32]) -> ConstitutionalSnapshot {
    ConstitutionalSnapshot {
        epoch,
        height: epoch * 100,
        state_root: [0xAA; 32],
        validator_root: [0xBB; 32],
        execution_root: [0xCC; 32],
        previous_snapshot_hash: parent_hash,
        replay_certificate: ReplayCertificate {
            transcript: ReplayTranscript {
                tx_hashes: vec![],
                pre_state_roots: vec![],
                post_state_roots: vec![],
                receipts: vec![],
                emitted_events: vec![],
                scheduler_trace: vec![],
                consensus_trace: vec![],
            },
            proof: ReplayEquivalenceProof {
                live_root: [0xDD; 32],
                replayed_root: [0xDD; 32],
                identical: true,
            },
        },
        timestamp_slot: epoch * 1000,
    }
}

#[test]
fn test_snapshot_continuity_chain() {
    let genesis = make_snapshot(0, [0x00; 32]);
    let gen_hash = genesis.seal_hash();
    let snap1 = make_snapshot(1, gen_hash);
    assert!(ContinuityChain::verify_link(&gen_hash, &snap1));
}

#[test]
fn test_snapshot_continuity_rejects_broken_chain() {
    let genesis = make_snapshot(0, [0x00; 32]);
    let gen_hash = genesis.seal_hash();
    let snap1 = make_snapshot(1, [0xFF; 32]);
    assert!(!ContinuityChain::verify_link(&gen_hash, &snap1));
}
