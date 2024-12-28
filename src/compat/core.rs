#![allow(missing_docs)]

use crate::compat::{bool_atomic_impl, int_atomic_impl};
use crate::prelude::*;

bool_atomic_impl!(core_atomic::AtomicBool);
int_atomic_impl!(core_atomic::AtomicI8 => i8);
int_atomic_impl!(core_atomic::AtomicI16 => i16);
int_atomic_impl!(core_atomic::AtomicI32 => i32);
int_atomic_impl!(core_atomic::AtomicI64 => i64);
int_atomic_impl!(core_atomic::AtomicIsize => isize);
int_atomic_impl!(core_atomic::AtomicU8 => u8);
int_atomic_impl!(core_atomic::AtomicU16 => u16);
int_atomic_impl!(core_atomic::AtomicU32 => u32);
int_atomic_impl!(core_atomic::AtomicU64 => u64);
int_atomic_impl!(core_atomic::AtomicUsize => usize);

pub type AtomicBool = core_atomic::AtomicBool;
pub type AtomicI8 = core_atomic::AtomicI8;
pub type AtomicI16 = core_atomic::AtomicI16;
pub type AtomicI32 = core_atomic::AtomicI32;
pub type AtomicI64 = core_atomic::AtomicI64;
pub type AtomicIsize = core_atomic::AtomicIsize;
pub type AtomicU8 = core_atomic::AtomicU8;
pub type AtomicU16 = core_atomic::AtomicU16;
pub type AtomicU32 = core_atomic::AtomicU32;
pub type AtomicU64 = core_atomic::AtomicU64;
pub type AtomicUsize = core_atomic::AtomicUsize;

pub use core_atomic::fence;