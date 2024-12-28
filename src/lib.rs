//! Convenient tool for atomics in Rust.
//!
//! # Crate features
//! | Feature | Description |     |
//! | ------- | ----------- | --- |
//! | `compat_core` | Compatibility with `core::sync::atomic` | DEFAULT |
//! | `compat_loom` | Compatibility with `loom::sync::atomic` |         |
//! | `default_core`[^1] | Default implementation with `core::sync::atomic` | DEFAULT |
//! | `default_loom`[^1] | Default implementation with `loom::sync::atomic` |         |
//! 
//! [^1]: `default_<impl>` features are mutually exclusive.
//! 
//! # Usage
//! Usually, you would want to use the atomiq crate with the default implementation:
//! ```
//! use atomiq::default::AtomicBool;
//! 
//! let atomic = AtomicBool::new(false);
//! ```
#![no_std]
#![allow(unused)]
#![warn(missing_docs)]
extern crate alloc;

pub mod compat;
#[cfg(any(feature = "default_core", feature = "default_loom"))]
pub mod default;
#[cfg(any(feature = "default_core", feature = "default_loom"))]
pub mod option;
mod prelude;
mod ordering;
pub use ordering::{Ordering, OrderingExt};