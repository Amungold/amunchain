// Three-dimensional State<T, Structural, Durability, Consensus>.
// Each dimension transitions independently.

use crate::states::*;
use core::marker::PhantomData;

pub struct State<T, S = Unverified, D = Volatile, C = Proposed> {
    inner: T,
    _s: PhantomData<S>,
    _d: PhantomData<D>,
    _c: PhantomData<C>,
}

// Entry point
impl<T> State<T, Unverified, Volatile, Proposed> {
    pub fn new(value: T) -> Self {
        Self {
            inner: value,
            _s: PhantomData,
            _d: PhantomData,
            _c: PhantomData,
        }
    }
}

// Accessors
impl<T, S, D, C> State<T, S, D, C> {
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

// Structural transitions
impl<T, D, C> State<T, Unverified, D, C> {
    pub fn verify<F, E>(self, f: F) -> Result<State<T, Verified, D, C>, (Self, E)>
    where
        F: FnOnce(&T) -> Result<(), E>,
    {
        match f(&self.inner) {
            Ok(()) => Ok(State {
                inner: self.inner,
                _s: PhantomData,
                _d: PhantomData,
                _c: PhantomData,
            }),
            Err(e) => Err((self, e)),
        }
    }
}

impl<T, D, C> State<T, Verified, D, C> {
    pub fn commit(self) -> State<T, Committed, D, C> {
        State {
            inner: self.inner,
            _s: PhantomData,
            _d: PhantomData,
            _c: PhantomData,
        }
    }
}

impl<T, D, C> State<T, Committed, D, C> {
    pub fn finalize(self) -> State<T, Finalized, D, C> {
        State {
            inner: self.inner,
            _s: PhantomData,
            _d: PhantomData,
            _c: PhantomData,
        }
    }
}

// Durability transitions
impl<T, S, C> State<T, S, Volatile, C> {
    pub fn make_durable(self) -> State<T, S, Durable, C> {
        State {
            inner: self.inner,
            _s: PhantomData,
            _d: PhantomData,
            _c: PhantomData,
        }
    }
}

impl<T, S, C> State<T, S, Durable, C> {
    pub fn journal(self) -> State<T, S, Journaled, C> {
        State {
            inner: self.inner,
            _s: PhantomData,
            _d: PhantomData,
            _c: PhantomData,
        }
    }
}

// Consensus transitions
impl<T, S, D> State<T, S, D, Proposed> {
    pub fn mark_voted(self) -> State<T, S, D, Voted> {
        State {
            inner: self.inner,
            _s: PhantomData,
            _d: PhantomData,
            _c: PhantomData,
        }
    }
}

impl<T, S, D> State<T, S, D, Voted> {
    pub fn mark_quorum_certified(self) -> State<T, S, D, QuorumCertified> {
        State {
            inner: self.inner,
            _s: PhantomData,
            _d: PhantomData,
            _c: PhantomData,
        }
    }
}

impl<T, S, D> State<T, S, D, QuorumCertified> {
    pub fn mark_executed(self) -> State<T, S, D, Executed> {
        State {
            inner: self.inner,
            _s: PhantomData,
            _d: PhantomData,
            _c: PhantomData,
        }
    }
}

// Extraction only from Finalized state
impl<T, D, C> State<T, Finalized, D, C> {
    pub fn into_inner(self) -> T {
        self.inner
    }
}
