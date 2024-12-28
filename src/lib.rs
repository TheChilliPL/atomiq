//! Convenient tool for atomics in Rust.
//!
//! # Features
//!
//! - Common atomic traits and types.
//! - Standard library/core implementation (default `compat-core` create feature)
//! - [Loom][loom] implementation for testing (`compat-loom` crate feature)
//! - Avoiding generics by providing default implementation (either `default-core` (default) or
//! `default-loom` crate features)
//! - Atomic option type (requires default implementation)
//!
//! [loom]: https://docs.rs/loom
#![no_std]
#![allow(unused)]
extern crate alloc;

pub mod compat;
#[cfg(any(feature = "default_core", feature = "default_loom"))]
pub mod default;
#[cfg(any(feature = "default_core", feature = "default_loom"))]
pub mod option;
mod prelude;
mod ordering;
pub use ordering::{Ordering, OrderingExt};