use amun_resource_core::ResourceId;
use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
use crate::gas_meter::GasMeter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GasEngineResult {
    Success { gas_used: u64 },
    OutOfGas { gas_used: u64, gas_limit: u64 },
}

pub struct GasEngine;

impl GasEngine {
    pub fn execute_with_gas<F>(
        gas_limit: u64,
        contract_id: ResourceId,
        block_height: u64,
        transaction_hash: [u8; 32],
        execute_fn: F,
    ) -> (GasEngineResult, Option<ConstitutionalEvidence>)
    where
        F: FnOnce(&mut GasMeter) -> Result<(), String>,
    {
        let mut meter = GasMeter::new(gas_limit);
        match execute_fn(&mut meter) {
            Ok(()) => {
                if meter.is_exhausted() {
                    let ev = ConstitutionalEvidence::ExecutionFailure {
                        reason: "out of gas".into(),
                        contract_id,
                        block_height,
                        transaction_hash,
                        gas_consumed: meter.gas_used,
                    };
                    (GasEngineResult::OutOfGas { gas_used: meter.gas_used, gas_limit }, Some(ev))
                } else {
                    (GasEngineResult::Success { gas_used: meter.gas_used }, None)
                }
            }
            Err(reason) => {
                let ev = ConstitutionalEvidence::ExecutionFailure {
                    reason,
                    contract_id,
                    block_height,
                    transaction_hash,
                    gas_consumed: meter.gas_used,
                };
                (GasEngineResult::OutOfGas { gas_used: meter.gas_used, gas_limit }, Some(ev))
            }
        }
    }

    pub fn can_execute(gas_limit: u64, estimated_cost: u64) -> bool {
        gas_limit >= estimated_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_resource_core::ResourceId;

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32]; h[0] = seed; ResourceId(h)
    }

    #[test]
    fn w7_execute_within_gas() {
        let (result, evidence) = GasEngine::execute_with_gas(
            1000, make_id(1), 1, [0xaa; 32],
            |meter| { meter.charge(100)?; meter.charge(200)?; Ok(()) },
        );
        assert!(matches!(result, GasEngineResult::Success { gas_used: 300 }));
        assert!(evidence.is_none());
    }

    #[test]
    fn w7_execute_out_of_gas_produces_evidence() {
        let (result, evidence) = GasEngine::execute_with_gas(
            100, make_id(2), 42, [0xbb; 32],
            |meter| { meter.charge(80)?; meter.charge(30)?; Ok(()) },
        );
        assert!(matches!(result, GasEngineResult::OutOfGas { gas_used: 100, gas_limit: 100 }));
        assert!(evidence.is_some());
    }

    #[test]
    fn w7_gas_estimation_blocks_execution() {
        assert!(GasEngine::can_execute(1000, 500));
        assert!(!GasEngine::can_execute(100, 500));
    }

    #[test]
    fn w7_no_evidence_on_success() {
        let (result, evidence) = GasEngine::execute_with_gas(
            500, make_id(3), 10, [0xcc; 32],
            |meter| { meter.charge(50)?; Ok(()) },
        );
        assert!(matches!(result, GasEngineResult::Success { .. }));
        assert!(evidence.is_none());
    }

    #[test]
    fn w7_gas_accounting_deterministic() {
        let run = || GasEngine::execute_with_gas(
            200, make_id(4), 1, [0xdd; 32],
            |meter| { meter.charge(30)?; meter.charge(40)?; meter.charge(50)?; Ok(()) },
        );
        let (r1, _) = run();
        let (r2, _) = run();
        assert_eq!(r1, r2);
    }
}
