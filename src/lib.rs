//! Convenient tool for atomics in Rust.
//!
//! # Crate features
//! `alloc` --- enables the `Arc` type. (default)
//! `derive` --- enables the derive macros. (default)
//! `loom` --- replaces the default implementation with the `loom` mock.
//!
//! # Usage
//! ```
//! use atomiq::prelude::*;
//! # use atomiq::try_init_model;
//!
//! # try_init_model(|| {
//! let atomic: Atomic<bool> = Atomic::from(false);
//! atomic.store(true, Ordering::Release);
//! assert_eq!(atomic.load(Ordering::Acquire), true);
//! # });
//! ```
#![no_std]
#![allow(unused)]
#![warn(missing_docs)]

pub mod option;
pub mod prelude;
mod ordering;
mod atomic;
mod atom;
mod atomizable;
mod try_init_model;

pub use atomic::Atomic;
pub use atom::{Atom, BitAtom, IntAtom};
pub use atomizable::{Atomizable, BitAtomizable, IntAtomizable, Atomize};
pub use ordering::{Ordering, OrderingExt};
pub use try_init_model::try_init_model;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
pub use atom::Arc;

#[cfg(feature = "derive")]
pub use atomiq_derive as derive;