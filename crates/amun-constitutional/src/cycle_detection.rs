//! Cycle Detection — formal DAG enforcement for causal graphs.
//!
//! The constitutional causal graph MUST be acyclic.
//! This module provides detection and classification of cycles.

use crate::causal_edge::CausalEdge;
use crate::kernel_types::ConstitutionalHash;
use crate::prelude::*;

/// Result of cycle detection on a causal graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CycleDetectionResult {
    Acyclic,
    SelfLoop {
        artifact_hash: ConstitutionalHash,
    },
    CycleDetected {
        cycle_path: Vec<ConstitutionalHash>,
        cycle_length: usize,
    },
}

impl CycleDetectionResult {
    pub fn is_acyclic(&self) -> bool {
        matches!(self, CycleDetectionResult::Acyclic)
    }
    pub fn has_cycle(&self) -> bool {
        !self.is_acyclic()
    }
}

/// Detect cycles using DFS with coloring (white=0, gray=1, black=2).
pub fn detect_cycles(edges: &[CausalEdge]) -> CycleDetectionResult {
    use crate::prelude::*;

    let mut all_hashes: Vec<ConstitutionalHash> = Vec::new();
    for edge in edges {
        if !all_hashes.contains(&edge.source_hash) {
            all_hashes.push(edge.source_hash);
        }
        if !all_hashes.contains(&edge.target_hash) {
            all_hashes.push(edge.target_hash);
        }
    }

    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); all_hashes.len()];
    for edge in edges {
        if let (Some(src_idx), Some(tgt_idx)) = (
            all_hashes.iter().position(|h| *h == edge.source_hash),
            all_hashes.iter().position(|h| *h == edge.target_hash),
        ) {
            if src_idx == tgt_idx {
                return CycleDetectionResult::SelfLoop {
                    artifact_hash: edge.source_hash,
                };
            }
            adjacency[src_idx].push(tgt_idx);
        }
    }

    let mut color: Vec<u8> = vec![0; all_hashes.len()];
    let mut path: Vec<usize> = Vec::new();

    for start in 0..all_hashes.len() {
        if color[start] == 0 {
            if let Some(cycle) = dfs_visit(start, &adjacency, &mut color, &mut path, &all_hashes) {
                return cycle;
            }
        }
    }
    CycleDetectionResult::Acyclic
}

fn dfs_visit(
    node: usize,
    adjacency: &[Vec<usize>],
    color: &mut Vec<u8>,
    path: &mut Vec<usize>,
    all_hashes: &[ConstitutionalHash],
) -> Option<CycleDetectionResult> {
    color[node] = 1;
    path.push(node);
    for &neighbor in &adjacency[node] {
        if color[neighbor] == 1 {
            let cycle_start = path.iter().position(|&p| p == neighbor).unwrap();
            let cycle_path: Vec<ConstitutionalHash> = path[cycle_start..]
                .iter()
                .map(|&idx| all_hashes[idx])
                .collect();
            return Some(CycleDetectionResult::CycleDetected {
                cycle_path,
                cycle_length: path.len() - cycle_start + 1,
            });
        }
        if color[neighbor] == 0 {
            if let Some(cycle) = dfs_visit(neighbor, adjacency, color, path, all_hashes) {
                return Some(cycle);
            }
        }
    }
    color[node] = 2;
    path.pop();
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::causality_type::CausalityType;

    // Use the ACTUAL CausalEdge::new signature from causal_edge.rs
    fn make_edge(src: [u8; 32], tgt: [u8; 32]) -> CausalEdge {
        CausalEdge::new(
            1,
            1,
            1,
            src,
            tgt,
            CausalityType::ConstitutionalDependency,
            [0xAB; 32],
        )
    }

    #[test]
    fn test_acyclic() {
        let e = vec![
            make_edge([0x01; 32], [0x02; 32]),
            make_edge([0x02; 32], [0x03; 32]),
            make_edge([0x01; 32], [0x03; 32]),
        ];
        assert!(detect_cycles(&e).is_acyclic());
    }
    #[test]
    fn test_self_loop() {
        let e = vec![make_edge([0x01; 32], [0x01; 32])];
        assert!(detect_cycles(&e).has_cycle());
    }
    #[test]
    fn test_simple_cycle() {
        let e = vec![
            make_edge([0x01; 32], [0x02; 32]),
            make_edge([0x02; 32], [0x03; 32]),
            make_edge([0x03; 32], [0x01; 32]),
        ];
        assert!(detect_cycles(&e).has_cycle());
    }
    #[test]
    fn test_single_edge() {
        assert!(detect_cycles(&[make_edge([0x01; 32], [0x02; 32])]).is_acyclic());
    }
    #[test]
    fn test_empty() {
        assert!(detect_cycles(&[]).is_acyclic());
    }
}
