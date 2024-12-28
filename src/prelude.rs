//! Internal prelude for atomiq. Makes it less likely to mistake different atomic types.
pub use crate::{Ordering, OrderingExt};
#[cfg(feature = "compat_core")]
pub use core::sync::atomic as core_atomic;
#[cfg(feature = "compat_loom")]
pub use loom::sync::atomic as loom_atomic;
