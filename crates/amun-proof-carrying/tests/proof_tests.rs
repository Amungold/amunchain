use amun_constitutional_kernel::receipt::ExecutionReceipt;
use amun_constitutional_commitments::SparseMerkleTree;
use amun_constitutional_block::BlockBuilder;
use amun_proof_carrying::ProofCarryingReceipt;

fn dummy_receipt(id: &str) -> ExecutionReceipt {
    ExecutionReceipt::new(id.into(), "s".into(), 0, 1, "t".into())
}

#[test]
fn test_proof_carrying_receipt_creation() {
    let mut st = SparseMerkleTree::new(b"AMUN_STATE_DOMAIN");
    let gov = SparseMerkleTree::new(b"AMUN_GOVERNANCE_DOMAIN");
    let exec_tree = SparseMerkleTree::new(b"AMUN_EXECUTION_DOMAIN");
    let evidence_tree = SparseMerkleTree::new(b"AMUN_EVIDENCE_DOMAIN");
    st.insert(b"key", &[42u8; 32]);
    let r = dummy_receipt("r1");
    let proof = st.prove(b"key");
    let block = BlockBuilder::build(0, "0".repeat(64), "t".into(), "p".into(), vec![r.clone()], &st, &gov, &exec_tree, &evidence_tree, String::new());
    let pcr = ProofCarryingReceipt::new(r, proof, None, None, block.state_root.clone(), block.governance_root.clone(), block.execution_root.clone(), block.block_hash.clone());
    assert!(!pcr.receipt.receipt_id.is_empty());
}
