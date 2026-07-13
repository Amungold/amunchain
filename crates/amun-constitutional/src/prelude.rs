#[cfg(not(feature = "std"))]
pub use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
    vec::Vec,
};

#[cfg(feature = "std")]
pub use std::{
    collections::{BTreeMap, BTreeSet},
    string::String,
    vec::Vec,
};
