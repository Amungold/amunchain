use amun_kernel_types::PublicHash32;
use amun_block::Block;
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use amun_constitution::gas::{GasMeter, gas_costs};
use crate::state::StateStore;
use crate::receipt::ExecutionReceipt;

pub struct StateTransition;

impl StateTransition {
    pub fn apply_block<S: StateStore>(
        store: &mut S,
        block: &Block,
    ) -> AmunResult<(PublicHash32, heapless::Vec<ExecutionReceipt, 64>)> {
        let mut receipts = heapless::Vec::new();
        let mut block_gas = GasMeter::new(gas_costs::MAX_GAS_PER_BLOCK);

        for (i, tx_hash) in block.body.tx_hashes.iter().enumerate() {
            if block_gas.consume(gas_costs::TX_BASE).is_err() {
                return Err(FailureContext::new(
                    ConstitutionalFault::CryptoBudgetExceeded, 0x0008, 0x0101));
            }
            let receipt = ExecutionReceipt::new(
                *tx_hash, i as u32, gas_costs::TX_BASE,
                PublicHash32::default(), 0,
            );
            receipts.push(receipt).map_err(|_| FailureContext::new(
                ConstitutionalFault::CapacityExceeded, 0x0008, 0x0100))?;
        }

        let root = store.root()?;
        Ok((root, receipts))
    }
}
