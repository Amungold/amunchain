// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use amun_constitution_builder::digest::ArtifactDigest;
use amun_constitution_builder::canonical_bytes::CanonicalSerialize;
use crate::quorum::QuorumPolicy;

/// A proposal that is put to a constitutional vote.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Proposal {
    pub proposal_id: String,
    pub title: String,
    pub description: String,
    pub epoch_start: String,
    pub epoch_end: String,
    pub quorum: QuorumPolicy,
}

impl ArtifactDigest for Proposal {
    fn domain_separator(&self) -> &'static [u8] {
        b"AMUN_PROPOSAL_V1"
    }
}

impl Proposal {
    pub fn new(
        title: String,
        description: String,
        epoch_start: String,
        epoch_end: String,
        quorum: QuorumPolicy,
    ) -> Self {
        let mut tmp = Self {
            proposal_id: String::new(),
            title,
            description,
            epoch_start,
            epoch_end,
            quorum,
        };
        let id = tmp.compute_id();
        tmp.proposal_id = id;
        tmp
    }

    fn identity_bytes(&self) -> Vec<u8> {
        let mut c = self.clone();
        c.proposal_id = String::new();
        serde_json::to_vec(&c).expect("Proposal serialization must not fail")
    }

    fn compute_id(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_PROPOSAL_V1");
        hasher.update(&self.identity_bytes());
        hex::encode(hasher.finalize().as_bytes())
    }
}

impl CanonicalSerialize for Proposal {
    fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Canonical serialization must not fail")
    }
}

/// A ballot represents a single vote from a constitutional participant.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ballot {
    pub proposal_id: String,
    pub voter_public_key_hex: String,
    pub approval: bool,
    pub timestamp: String,
}

/// A tally is the deterministic result of counting ballots.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tally {
    pub proposal_id: String,
    pub total_participants: u64,
    pub approvals: u64,
    pub passed: bool,
}

impl Tally {
    /// Compute the tally from a list of ballots, using the proposal's quorum
    /// and a set of eligible voter keys.  The result is deterministic because
    /// ballots are processed in sorted order.
    pub fn compute(proposal: &Proposal, ballots: &[Ballot], eligible_voters: &[String]) -> Self {
        let mut sorted_ballots = ballots.to_vec();
        sorted_ballots.sort_by(|a, b| a.voter_public_key_hex.cmp(&b.voter_public_key_hex));

        let mut seen = BTreeMap::new();
        for ballot in &sorted_ballots {
            // Ignore ballots from non-eligible voters or duplicate voters (first seen wins).
            if !eligible_voters.contains(&ballot.voter_public_key_hex) {
                continue;
            }
            if seen.contains_key(&ballot.voter_public_key_hex) {
                continue;
            }
            seen.insert(ballot.voter_public_key_hex.clone(), ballot.approval);
        }

        let total_participants = seen.len() as u64;
        let approvals = seen.values().filter(|&&v| v).count() as u64;
        let passed = proposal.quorum.is_satisfied(total_participants, approvals);

        Self {
            proposal_id: proposal.proposal_id.clone(),
            total_participants,
            approvals,
            passed,
        }
    }
}
