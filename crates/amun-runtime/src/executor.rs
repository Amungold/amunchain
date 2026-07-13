use crate::overlay::OverlayState;
use amun_failure::AmunResult;
use heapless::Vec;

pub struct AtomicExecutor;

impl AtomicExecutor {
    pub fn apply_transaction(
        overlay: &mut OverlayState,
        key: Vec<u8, 32>,
        value: Vec<u8, 32>,
    ) -> AmunResult<()> {
        overlay.set(key, value)
    }

    pub fn execute_batch(
        overlay: &mut OverlayState,
        ops: &[(Vec<u8, 32>, Option<Vec<u8, 32>>)],
    ) -> AmunResult<()> {
        for (k, v) in ops {
            match v {
                Some(val) => overlay.set(k.clone(), val.clone())?,
                None => overlay.delete(k.clone())?,
            }
        }
        Ok(())
    }
}
