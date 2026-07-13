// Single-element uninitialized memory slot with explicit state tracking.
// State is always tracked via a Cell<u8> — no difference between debug and release.

use core::mem::MaybeUninit;
use core::ptr;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum SlotState {
    Uninit = 0,
    Init = 1,
    Moved = 2,
    Poisoned = 255,
}

pub struct RawSlot<T> {
    data: MaybeUninit<T>,
    state: core::cell::Cell<u8>,
}

impl<T> RawSlot<T> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            data: MaybeUninit::uninit(),
            state: core::cell::Cell::new(SlotState::Uninit as u8),
        }
    }

    #[inline(always)]
    fn get_state(&self) -> SlotState {
        match self.state.get() {
            0 => SlotState::Uninit,
            1 => SlotState::Init,
            2 => SlotState::Moved,
            255 => SlotState::Poisoned,
            _ => {
                self.state.set(SlotState::Poisoned as u8);
                SlotState::Poisoned
            }
        }
    }

    #[inline(always)]
    fn set_state(&self, s: SlotState) {
        self.state.set(s as u8);
    }

    pub fn write(&mut self, value: T) -> Result<(), &'static str> {
        match self.get_state() {
            SlotState::Uninit | SlotState::Moved => {
                unsafe {
                    self.data.as_mut_ptr().write(value);
                }
                self.set_state(SlotState::Init);
                Ok(())
            }
            SlotState::Init => Err("RawSlot::write: slot already initialized"),
            SlotState::Poisoned => Err("RawSlot::write: slot is poisoned"),
        }
    }

    pub fn take(&mut self) -> Result<T, &'static str> {
        match self.get_state() {
            SlotState::Init => {
                self.set_state(SlotState::Moved);
                let value = unsafe { self.data.assume_init_read() };
                self.data = MaybeUninit::uninit();
                self.set_state(SlotState::Uninit);
                Ok(value)
            }
            SlotState::Uninit => Err("RawSlot::take: slot not initialized"),
            SlotState::Moved => Err("RawSlot::take: slot already moved"),
            SlotState::Poisoned => Err("RawSlot::take: slot is poisoned"),
        }
    }

    pub fn replace(&mut self, value: T) -> Result<T, &'static str> {
        match self.get_state() {
            SlotState::Init => {
                self.set_state(SlotState::Moved);
                let old = unsafe { self.data.assume_init_read() };
                unsafe {
                    self.data.as_mut_ptr().write(value);
                }
                self.set_state(SlotState::Init);
                Ok(old)
            }
            _ => Err("RawSlot::replace: slot not initialized"),
        }
    }

    pub fn get(&self) -> Result<&T, &'static str> {
        match self.get_state() {
            SlotState::Init => Ok(unsafe { self.data.assume_init_ref() }),
            _ => Err("RawSlot::get: slot not initialized"),
        }
    }

    pub fn get_mut(&mut self) -> Result<&mut T, &'static str> {
        match self.get_state() {
            SlotState::Init => Ok(unsafe { self.data.assume_init_mut() }),
            _ => Err("RawSlot::get_mut: slot not initialized"),
        }
    }

    pub fn is_init(&self) -> bool {
        self.get_state() == SlotState::Init
    }
}

impl<T> Drop for RawSlot<T> {
    fn drop(&mut self) {
        if self.get_state() == SlotState::Init {
            unsafe {
                ptr::drop_in_place(self.data.as_mut_ptr());
            }
        }
    }
}

impl<T> Default for RawSlot<T> {
    fn default() -> Self {
        Self::new()
    }
}
