//! The prelude module contains all the types and traits that are commonly used in the `atomiq` 
//! crate.

pub use crate::{Ordering, OrderingExt};

pub use crate::atomic::Atomic;
pub use crate::atom::*;
pub use crate::atomizable::{Atomizable, BitAtomizable, IntAtomizable, Atomize};
pub use crate::CancellationToken;

#[cfg(feature = "derive")]
pub use crate::derive::*;