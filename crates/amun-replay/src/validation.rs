use crate::commit_log::{StateCommit, CommitLog};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayResult {
    pub expected_root: [u8; 32],
    pub replayed_root: [u8; 32],
    pub valid: bool,
    pub commits_checked: usize,
}

impl ReplayResult {
    pub fn success(expected: [u8; 32], count: usize) -> Self {
        Self { expected_root: expected, replayed_root: expected, valid: true, commits_checked: count }
    }

    pub fn failure(expected: [u8; 32], actual: [u8; 32], count: usize) -> Self {
        Self { expected_root: expected, replayed_root: actual, valid: false, commits_checked: count }
    }
}

pub struct ReplayValidator;

impl ReplayValidator {
    pub fn validate(commits: &[StateCommit]) -> ReplayResult {
        if commits.is_empty() {
            return ReplayResult::success([0u8; 32], 0);
        }
        for i in 1..commits.len() {
            let prev = &commits[i - 1];
            let curr = &commits[i];
            if curr.previous_root != prev.new_root {
                return ReplayResult::failure(prev.new_root, curr.previous_root, i);
            }
        }
        let last = commits.last().expect("validation: invariant violated — non-empty after check");
        ReplayResult::success(last.new_root, commits.len())
    }

    pub fn validate_log(log: &CommitLog) -> ReplayResult {
        Self::validate(&log.commits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_commit(h: u64, prev: [u8; 32], new: [u8; 32]) -> StateCommit {
        StateCommit {
            height: h,
            block_hash: [h as u8; 32],
            previous_root: prev,
            new_root: new,
            tx_count: 1,
            timestamp: h * 1000,
        }
    }

    #[test]
    fn n35_valid_chain() {
        let commits = vec![
            make_commit(1, [0u8; 32], [10u8; 32]),
            make_commit(2, [10u8; 32], [20u8; 32]),
            make_commit(3, [20u8; 32], [30u8; 32]),
        ];
        let result = ReplayValidator::validate(&commits);
        assert!(result.valid);
        assert_eq!(result.commits_checked, 3);
    }

    #[test]
    fn n35_broken_chain() {
        let commits = vec![
            make_commit(1, [0u8; 32], [10u8; 32]),
            make_commit(2, [99u8; 32], [20u8; 32]),
        ];
        let result = ReplayValidator::validate(&commits);
        assert!(!result.valid);
        assert_eq!(result.commits_checked, 1);
    }

    #[test]
    fn n35_single_commit() {
        let commits = vec![make_commit(1, [0u8; 32], [10u8; 32])];
        let result = ReplayValidator::validate(&commits);
        assert!(result.valid);
        assert_eq!(result.commits_checked, 1);
    }

    #[test]
    fn n35_empty_commits() {
        let result = ReplayValidator::validate(&[]);
        assert!(result.valid);
        assert_eq!(result.commits_checked, 0);
    }

    #[test]
    fn n35_replay_result_deterministic() {
        let commits = vec![
            make_commit(1, [0u8; 32], [10u8; 32]),
            make_commit(2, [10u8; 32], [20u8; 32]),
        ];
        let r1 = ReplayValidator::validate(&commits);
        let r2 = ReplayValidator::validate(&commits);
        assert_eq!(r1, r2);
    }
}
