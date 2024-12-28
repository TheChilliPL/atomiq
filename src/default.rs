//! Reexports the default implementation of the library, either `compat-core` or `compat-loom`.
//! 
//! # Examples
//! ```
//! // Import the default implementation with whatever you need.
//! use atomiq::default::AtomicBool;
//! 
//! let atomic = AtomicBool::new(false);
//! ```

#[cfg(all(feature = "default_core", feature = "default_loom"))]
compile_error!("Cannot enable both `default-core` and `default-loom` features");

#[cfg(feature = "default_core")]
pub use crate::compat::core::*;

#[cfg(feature = "default_loom")]
pub use crate::compat::loom::*;
