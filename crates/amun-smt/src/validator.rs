//! Structural validator for SMT invariants.
//!
//! Uses manual path push/pop for cycle detection.
//! This is a consensus-critical component — any invariant violation
//! detected here indicates a bug in the tree implementation.

use crate::hash::{Hash, bit, max_skip_len};
use crate::node::{Node, EMPTY_NODE_HASH};
use crate::context::Context;
use crate::error::SmtError;

/// Validate all structural invariants for the tree rooted at `root`.
pub fn validate_tree(root: &Hash, ctx: &Context) -> Result<(), SmtError> {
    if *root == *EMPTY_NODE_HASH {
        return Ok(());
    }
    let mut path = Vec::new();
    validate_node(root, 0, ctx, &mut path)
}

fn validate_node(
    hash: &Hash,
    depth: usize,
    ctx: &Context,
    path: &mut Vec<Hash>,
) -> Result<(), SmtError> {
    if depth > 255 {
        return Err(SmtError::DepthOverflow { depth });
    }
    if *hash == *EMPTY_NODE_HASH {
        return Err(SmtError::EmptyChild { depth });
    }
    if path.contains(hash) {
        return Err(SmtError::CycleDetected { depth });
    }

    let node = ctx.get_node(hash)?;
    match node.as_ref() {
        Node::Leaf { .. } => Ok(()),
        Node::Branch {
            skip_len,
            prefix,
            left,
            right,
        } => {
            // Invariant: no empty children
            if *left == *EMPTY_NODE_HASH || *right == *EMPTY_NODE_HASH {
                return Err(SmtError::EmptyChild { depth });
            }

            // Invariant: skip_len within bounds
            let max = max_skip_len(depth);
            if *skip_len > max {
                return Err(SmtError::SkipLenTooLarge {
                    skip_len: *skip_len,
                    max,
                    depth,
                });
            }

            let next = depth + *skip_len as usize + 1;
            if next > 256 {
                return Err(SmtError::DepthOverflow { depth: next });
            }

            // Validate children with manual path tracking
            path.push(*hash);
            let result = (|| -> Result<(), SmtError> {
                validate_node(left, next, ctx, path)?;
                validate_node(right, next, ctx, path)?;
                Ok(())
            })();
            path.pop();
            result?;

            // Verify partition
            let decision_idx = depth + *skip_len as usize;
            let lmin = get_min_key(left, ctx)?;
            let lmax = get_max_key(left, ctx)?;
            let rmin = get_min_key(right, ctx)?;
            let rmax = get_max_key(right, ctx)?;

            // Full partition check: lmax < rmin
            if lmax >= rmin {
                return Err(SmtError::PartitionViolation {
                    key_pos: "lmax >= rmin".into(),
                    depth: decision_idx,
                });
            }
            if bit(&lmin, decision_idx) != 0 || bit(&lmax, decision_idx) != 0 {
                return Err(SmtError::PartitionViolation {
                    key_pos: "left".into(),
                    depth: decision_idx,
                });
            }
            if bit(&rmin, decision_idx) != 1 || bit(&rmax, decision_idx) != 1 {
                return Err(SmtError::PartitionViolation {
                    key_pos: "right".into(),
                    depth: decision_idx,
                });
            }

            // Verify prefix consistency
            for i in 0..*skip_len as usize {
                let expected = (prefix[i / 8] >> (7 - (i % 8))) & 1;
                if bit(&lmin, depth + i) != expected
                    || bit(&lmax, depth + i) != expected
                    || bit(&rmin, depth + i) != expected
                    || bit(&rmax, depth + i) != expected
                {
                    return Err(SmtError::PrefixMismatch {
                        bit_idx: depth + i,
                    });
                }
            }

            // Invariant: maximal skip (canonical minimality)
            let min_common = common_prefix_len_min(&lmin, &rmin, depth);
            if *skip_len != min_common {
                return Err(SmtError::SkipLenNotMinimal {
                    skip_len: *skip_len,
                    min: min_common,
                });
            }

            Ok(())
        }
    }
}

fn get_min_key(hash: &Hash, ctx: &Context) -> Result<[u8; 32], SmtError> {
    let node = ctx.get_node(hash)?;
    match node.as_ref() {
        Node::Leaf { key_hash, .. } => Ok(*key_hash),
        Node::Branch { left, .. } => get_min_key(left, ctx),
    }
}

fn get_max_key(hash: &Hash, ctx: &Context) -> Result<[u8; 32], SmtError> {
    let node = ctx.get_node(hash)?;
    match node.as_ref() {
        Node::Leaf { key_hash, .. } => Ok(*key_hash),
        Node::Branch { right, .. } => get_max_key(right, ctx),
    }
}

fn common_prefix_len_min(a: &[u8; 32], b: &[u8; 32], depth: usize) -> u8 {
    let mut len = 0;
    for i in depth..256 {
        if bit(a, i) != bit(b, i) {
            break;
        }
        len += 1;
    }
    len
}
