#[cfg(test)]
use crate::transitions::State;

#[test]
fn test_state_new() {
    let s: State<u64> = State::new(42);
    assert_eq!(*s.inner(), 42);
}

#[test]
fn test_state_verify_success() {
    let s: State<u64> = State::new(42);

    let verified = match s.verify(|v| if *v > 0 { Ok(()) } else { Err("zero") }) {
        Ok(v) => v,
        Err(_) => panic!("verification should succeed"),
    };

    assert_eq!(*verified.inner(), 42);
}

#[test]
fn test_state_verify_failure() {
    let s: State<u64> = State::new(0);

    let result = s.verify(|v| {
        if *v > 0 {
            Ok(())
        } else {
            Err("must be positive")
        }
    });

    assert!(result.is_err());
}

#[test]
fn test_state_commit() {
    let s: State<u64> = State::new(42);

    let verified = match s.verify(|_| Ok::<(), ()>(())) {
        Ok(v) => v,
        Err(_) => panic!("verification should succeed"),
    };

    let committed = verified.commit();

    assert_eq!(*committed.inner(), 42);
}

#[test]
fn test_state_finalize() {
    let s: State<u64> = State::new(42);

    let verified = match s.verify(|_| Ok::<(), ()>(())) {
        Ok(v) => v,
        Err(_) => panic!("verification should succeed"),
    };

    let finalized = verified.commit().finalize();

    assert_eq!(*finalized.inner(), 42);
}

#[test]
fn test_state_into_inner() {
    let s: State<u64> = State::new(42);

    let verified = match s.verify(|_| Ok::<(), ()>(())) {
        Ok(v) => v,
        Err(_) => panic!("verification should succeed"),
    };

    let finalized = verified.commit().finalize();

    assert_eq!(finalized.into_inner(), 42);
}

#[test]
fn test_state_make_durable() {
    let s: State<u64> = State::new(99);
    let durable = s.make_durable();
    assert_eq!(*durable.inner(), 99);
}

#[test]
fn test_state_mark_voted() {
    let s: State<u64> = State::new(42);

    let verified = match s.verify(|_| Ok::<(), ()>(())) {
        Ok(v) => v,
        Err(_) => panic!("verification should succeed"),
    };

    let voted = verified.mark_voted();

    assert_eq!(*voted.inner(), 42);
}
