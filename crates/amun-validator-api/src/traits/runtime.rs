use crate::error::PlatformResult;
use crate::types::state::RuntimeState;

pub trait ValidatorRuntime: Send + Sync {
    fn state(&self) -> PlatformResult<RuntimeState>;
    fn transition(&self, new_state: RuntimeState) -> PlatformResult<()>;
    fn start(&self) -> PlatformResult<()>;
    fn stop(&self) -> PlatformResult<()>;
    fn is_healthy(&self) -> PlatformResult<bool>;
}
