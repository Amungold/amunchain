#[cfg(test)]
use crate::guard::InitGuard;
use crate::slot::RawSlot;
use core::cell::Cell;

// RawSlot tests
#[test]
fn slot_new_is_empty() {
    let slot: RawSlot<u64> = RawSlot::new();
    assert!(!slot.is_init());
}

#[test]
fn slot_write_take() {
    let mut slot = RawSlot::new();
    slot.write(42u64).expect("test invariant");
    assert!(slot.is_init());
    assert_eq!(slot.take().expect("test invariant"), 42);
    assert!(!slot.is_init());
}

#[test]
fn slot_write_get() {
    let mut slot = RawSlot::new();
    slot.write(99u64).expect("test invariant");
    assert_eq!(*slot.get().expect("test invariant"), 99);
}

#[test]
fn slot_write_get_mut() {
    let mut slot = RawSlot::new();
    slot.write(10u64).expect("test invariant");
    *slot.get_mut().expect("test invariant") = 20;
    assert_eq!(slot.take().expect("test invariant"), 20);
}

#[test]
fn slot_replace() {
    let mut slot = RawSlot::new();
    slot.write(1u64).expect("test invariant");
    let old = slot.replace(2u64).expect("test invariant");
    assert_eq!(old, 1);
    assert_eq!(*slot.get().expect("test invariant"), 2);
}

#[test]
fn slot_double_write_rejected() {
    let mut slot = RawSlot::new();
    slot.write(1u64).expect("test invariant");
    assert!(slot.write(2u64).is_err());
}

#[test]
fn slot_take_empty_rejected() {
    let mut slot: RawSlot<u64> = RawSlot::new();
    assert!(slot.take().is_err());
}

#[test]
fn slot_get_empty_rejected() {
    let slot: RawSlot<u64> = RawSlot::new();
    assert!(slot.get().is_err());
}

#[test]
fn slot_reuse_after_take() {
    let mut slot = RawSlot::new();
    slot.write(1u64).expect("test invariant");
    let _ = slot.take().expect("test invariant");
    slot.write(2u64).expect("test invariant");
    assert_eq!(slot.take().expect("test invariant"), 2);
}

#[test]
fn slot_drop_releases_value() {
    let counter = Cell::new(0u32);
    struct DropCount<'a> {
        c: &'a Cell<u32>,
    }
    impl<'a> Drop for DropCount<'a> {
        fn drop(&mut self) {
            self.c.set(self.c.get() + 1);
        }
    }
    let mut slot = RawSlot::new();
    slot.write(DropCount { c: &counter })
        .expect("test invariant");
    drop(slot);
    assert_eq!(counter.get(), 1);
}

// InitGuard tests
#[test]
fn guard_take_returns_value() {
    let guard = InitGuard::new(42u64);
    assert_eq!(guard.take(), 42);
}

#[test]
fn guard_as_ref() {
    let guard = InitGuard::new(99u64);
    assert_eq!(*guard.as_ref(), 99);
    guard.take();
}

#[test]
fn guard_as_mut() {
    let mut guard = InitGuard::new(10u64);
    *guard.as_mut() = 20;
    assert_eq!(guard.take(), 20);
}

#[test]
fn guard_drops_if_not_taken() {
    let counter = Cell::new(0u32);
    struct DropCount<'a> {
        c: &'a Cell<u32>,
    }
    impl<'a> Drop for DropCount<'a> {
        fn drop(&mut self) {
            self.c.set(self.c.get() + 1);
        }
    }
    {
        let _guard = InitGuard::new(DropCount { c: &counter });
    }
    assert_eq!(counter.get(), 1);
}

#[test]
fn guard_does_not_drop_if_taken() {
    let counter = Cell::new(0u32);
    struct DropCount<'a> {
        c: &'a Cell<u32>,
    }
    impl<'a> Drop for DropCount<'a> {
        fn drop(&mut self) {
            self.c.set(self.c.get() + 1);
        }
    }
    {
        let guard = InitGuard::new(DropCount { c: &counter });
        let _taken = guard.take();
    }
    assert_eq!(counter.get(), 1);
}
