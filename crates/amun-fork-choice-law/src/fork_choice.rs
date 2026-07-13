use amun_quorum_certificate::QuorumCertificate;
use std::collections::HashSet;

pub fn select_best_qc(qcs: &[QuorumCertificate]) -> Option<&QuorumCertificate> {
    if qcs.is_empty() {
        return None;
    }
    let mut best = &qcs[0];
    for qc in &qcs[1..] {
        if qc.round > best.round || (qc.round == best.round && qc.block_hash > best.block_hash) {
            best = qc;
        }
    }
    Some(best)
}

pub fn qc_extends(qc: &QuorumCertificate, parent: &QuorumCertificate) -> bool {
    qc.parent_hash == parent.block_hash
}

pub fn find_chain<'a>(
    qc: &'a QuorumCertificate,
    all_qcs: &'a [QuorumCertificate],
) -> Vec<&'a QuorumCertificate> {
    let mut chain = vec![qc];
    let mut current_hash = qc.parent_hash;
    let mut visited = HashSet::new();
    visited.insert(qc.block_hash);

    let max_depth = 1000;
    let mut depth = 0;

    while depth < max_depth {
        if let Some(parent) = all_qcs.iter().find(|q| q.block_hash == current_hash) {
            if !visited.insert(parent.block_hash) {
                break;
            }
            chain.push(parent);
            current_hash = parent.parent_hash;
        } else {
            break;
        }
        depth += 1;
    }

    chain.reverse();
    chain
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_chain_position::ChainPosition;

    fn make_qc(
        round: u64,
        block_hash: [u8; 32],
        parent_hash: [u8; 32],
        height: u64,
    ) -> QuorumCertificate {
        QuorumCertificate::new(
            ChainPosition::new(0, height),
            round,
            block_hash,
            parent_hash,
            vec![],
        )
    }

    #[test]
    fn test_select_best_qc() {
        let qc1 = make_qc(1, [0x01; 32], [0x00; 32], 1);
        let qc2 = make_qc(2, [0x02; 32], [0x01; 32], 2);
        let qc3 = make_qc(2, [0x03; 32], [0x01; 32], 2);
        let qcs = vec![qc1, qc2, qc3];
        let best = select_best_qc(&qcs);
        assert!(best.is_some());
        assert_eq!(best.unwrap().block_hash, [0x03; 32]);
    }

    #[test]
    fn test_qc_extends() {
        let parent = make_qc(1, [0x01; 32], [0x00; 32], 1);
        let child = make_qc(2, [0x02; 32], [0x01; 32], 2);
        let unrelated = make_qc(2, [0x03; 32], [0xFF; 32], 2);
        assert!(qc_extends(&child, &parent));
        assert!(!qc_extends(&unrelated, &parent));
    }

    #[test]
    fn test_find_chain_simple() {
        let qc1 = make_qc(1, [0x01; 32], [0x00; 32], 1);
        let qc2 = make_qc(2, [0x02; 32], [0x01; 32], 2);
        let qc3 = make_qc(3, [0x03; 32], [0x02; 32], 3);
        let all = vec![qc1, qc2, qc3];
        let chain = find_chain(all.last().unwrap(), &all);
        assert!(chain.len() >= 2);
    }
}
