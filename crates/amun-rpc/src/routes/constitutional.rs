use axum::{extract::State, routing::get, Json, Router};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/constitutional/status", get(constitutional_status))
}

async fn constitutional_status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.lock().unwrap();
    let governance = state.governance.lock().unwrap();
    let authority = state.authority_registry.lock().unwrap();
    let kernel = state.constitutional_kernel.lock().unwrap();
    let slashing = state.slashing_ledger.lock().unwrap();
    let gossip = state.certificate_gossip.lock().unwrap();
    let economic = state.economic_ledger.lock().unwrap();
    let evidence_root = state.previous_evidence_root.lock().unwrap();

    Json(serde_json::json!({
        "consensus": {
            "height": engine.current_height,
            "validators": engine.total_validators,
            "active_validators": engine.active_validator_count(),
            "qcs_formed": engine.metrics.qcs_formed,
            "blocks_finalized": engine.metrics.blocks_finalized,
            "votes_received": engine.metrics.votes_received,
        },
        "governance": {
            "proposals": governance.proposals.len(),
            "votes": governance.votes.len()
        },
        "authority_registry": {
            "active_version": authority.active().map(|a| a.authority_version),
            "transition_pending": authority.transition.is_some()
        },
        "constitutional_kernel": {
            "constitutional_blocks": kernel.constitutional_count,
            "unconstitutional_blocks": kernel.unconstitutional_count,
            "compliance_ratio": kernel.compliance_ratio(),
            "active_laws": kernel.active_laws.len(),
            "verdict_history": kernel.verdict_history.len()
        },
        "slashing": {
            "executed": slashing.executed_count()
        },
        "certificate_gossip": {
            "pending": gossip.len()
        },
        "economic": {
            "treasury": economic.treasury(),
            "validator_pool": economic.validator_pool(),
            "ecosystem_pool": economic.ecosystem_pool(),
            "issued_supply": economic.issued_supply(),
            "burned_supply": economic.burned_supply(),
            "staked_supply": economic.staked_supply(),
            "economic_root": hex::encode(economic.compute_economic_root())
        },
        "evidence": {
            "previous_root": hex::encode(*evidence_root)
        }
    }))
}
