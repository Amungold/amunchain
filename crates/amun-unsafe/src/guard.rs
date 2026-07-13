// Panic-safe move wrapper. Uses ManuallyDrop to control drop semantics.
// If a panic occurs during a move, the guard ensures the value is dropped exactly once.

use core::mem::ManuallyDrop;

#[derive(Clone, Copy, PartialEq, Eq)]
enum GuardState {
    Active,
    Taken,
}

pub struct InitGuard<T> {
    value: ManuallyDrop<T>,
    state: GuardState,
}

impl<T> InitGuard<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: ManuallyDrop::new(value),
            state: GuardState::Active,
        }
    }

    pub fn take(mut self) -> T {
        self.state = GuardState::Taken;
        unsafe { ManuallyDrop::take(&mut self.value) }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn as_ref(&self) -> &T {
        &self.value
    }

    #[allow(clippy::should_implement_trait)]
    pub fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> Drop for InitGuard<T> {
    fn drop(&mut self) {
        if self.state == GuardState::Active {
            unsafe {
                ManuallyDrop::drop(&mut self.value);
            }
        }
    }
}
