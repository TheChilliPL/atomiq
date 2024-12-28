#![allow(missing_docs)]

use crate::compat::{bool_atomic_impl, int_atomic_impl};
use crate::prelude::*;

bool_atomic_impl!(loom_atomic::AtomicBool);
int_atomic_impl!(loom_atomic::AtomicI8 => i8);
int_atomic_impl!(loom_atomic::AtomicI16 => i16);
int_atomic_impl!(loom_atomic::AtomicI32 => i32);
int_atomic_impl!(loom_atomic::AtomicI64 => i64);
int_atomic_impl!(loom_atomic::AtomicIsize => isize);
int_atomic_impl!(loom_atomic::AtomicU8 => u8);
int_atomic_impl!(loom_atomic::AtomicU16 => u16);
int_atomic_impl!(loom_atomic::AtomicU32 => u32);
int_atomic_impl!(loom_atomic::AtomicU64 => u64);
int_atomic_impl!(loom_atomic::AtomicUsize => usize);

pub type AtomicBool = loom_atomic::AtomicBool;
pub type AtomicI8 = loom_atomic::AtomicI8;
pub type AtomicI16 = loom_atomic::AtomicI16;
pub type AtomicI32 = loom_atomic::AtomicI32;
pub type AtomicI64 = loom_atomic::AtomicI64;
pub type AtomicIsize = loom_atomic::AtomicIsize;
pub type AtomicU8 = loom_atomic::AtomicU8;
pub type AtomicU16 = loom_atomic::AtomicU16;
pub type AtomicU32 = loom_atomic::AtomicU32;
pub type AtomicU64 = loom_atomic::AtomicU64;
pub type AtomicUsize = loom_atomic::AtomicUsize;

pub use loom_atomic::fence;