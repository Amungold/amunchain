#![allow(dead_code)]
use amun_consensus::types::{BlockProposal, Vote, VoteType};
use amun_networking::envelope::Envelope;

pub fn build_proposal_envelope(node_id: &str, leader_id: [u8; 32], round: u64) -> Envelope {
    let proposal = BlockProposal {
        height: 1,
        block_hash: [0xAA; 32],
        proposer: leader_id,
        round,
        timestamp: 1000,
    };
    Envelope {
        sender: node_id.to_string(),
        recipient: String::new(),
        sequence: 1,
        timestamp: 1000,
        message_type: "proposal".into(),
        payload: serde_json::to_vec(&proposal).unwrap().into(),
    }
}
pub fn build_prevote_envelope(
    node_id: &str,
    voter_id: [u8; 32],
    block_hash: [u8; 32],
    round: u64,
) -> Envelope {
    let vote = Vote {
        height: 1,
        block_hash,
        voter: voter_id,
        round,
        vote_type: VoteType::Prevote,
        timestamp: 1001,
    };
    Envelope {
        sender: node_id.to_string(),
        recipient: String::new(),
        sequence: 2,
        timestamp: 1001,
        message_type: "prevote".into(),
        payload: serde_json::to_vec(&vote).unwrap().into(),
    }
}
pub fn build_precommit_envelope(
    node_id: &str,
    voter_id: [u8; 32],
    block_hash: [u8; 32],
    round: u64,
) -> Envelope {
    let vote = Vote {
        height: 1,
        block_hash,
        voter: voter_id,
        round,
        vote_type: VoteType::Precommit,
        timestamp: 1002,
    };
    Envelope {
        sender: node_id.to_string(),
        recipient: String::new(),
        sequence: 3,
        timestamp: 1002,
        message_type: "precommit".into(),
        payload: serde_json::to_vec(&vote).unwrap().into(),
    }
}

pub fn process_envelope(
    env: &Envelope,
    core: &mut super::SimulationNodeCore,
    validator_set: &amun_consensus::validator::ValidatorSet,
) {
    match env.message_type.as_str() {
        "proposal" => {
            if let Ok(p) = serde_json::from_slice::<BlockProposal>(&env.payload) {
                if p.round == core.state_machine.state.round {
                    if core.seen_proposals.contains(&p.block_hash) {
                        return;
                    }
                    core.seen_proposals.insert(p.block_hash);
                    core.state_machine.accept_proposal(p.block_hash);
                    core.last_proposal = Some(p);
                }
            }
        }
        "prevote" => {
            if let Ok(v) = serde_json::from_slice::<Vote>(&env.payload) {
                if v.round == core.state_machine.state.round {
                    core.state_machine.process_vote(v, validator_set);
                }
            }
        }
        "precommit" => {
            if let Ok(v) = serde_json::from_slice::<Vote>(&env.payload) {
                if v.round == core.state_machine.state.round {
                    core.state_machine.process_vote(v, validator_set);
                }
            }
        }
        _ => {}
    }
}
