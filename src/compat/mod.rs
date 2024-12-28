//! Atomic compatibility layers.

#[cfg(feature = "compat_core")]
pub mod core;
#[cfg(feature = "compat_loom")]
pub mod loom;

use crate::prelude::*;

/// Marks types that can be used like simple atomics.
///
/// Simple atomics only have to support load and store.
pub trait SimpleAtomic: From<Self::Value> + Default {
    type Value;

    fn new(value: Self::Value) -> Self {
        Self::from(value)
    }
    fn load(&self, ordering: Ordering) -> Self::Value;
    fn store(&self, value: Self::Value, ordering: Ordering);
}

/// Marks types that can be used like atomics.
pub trait Atomic: SimpleAtomic {
    // Deprecated
    // fn compare_and_swap(&self, current: Self::Value, new: Self::Value, ordering: Ordering) -> Self::Value;
    fn compare_exchange(&self, current: Self::Value, new: Self::Value, success: Ordering, failure: Ordering) -> Result<Self::Value, Self::Value>;
    fn compare_exchange_weak(&self, current: Self::Value, new: Self::Value, success: Ordering, failure: Ordering) -> Result<Self::Value, Self::Value>;
    fn fetch_update<F>(&self, set_ordering: Ordering, fetch_ordering: Ordering, f: F) -> Result<Self::Value, Self::Value>
    where
        F: FnMut(Self::Value) -> Option<Self::Value>;
    fn swap(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
}

/// Marks types that can be used like boolean atomics.
pub trait BoolAtomic: Atomic<Value = bool> {
    fn fetch_and(&self, value: bool, ordering: Ordering) -> bool;
    fn fetch_nand(&self, value: bool, ordering: Ordering) -> bool;
    // Loom does not support this for some reason
    // https://github.com/tokio-rs/loom/issues/371
    // fn fetch_not(&self, ordering: Ordering) -> bool;
    fn fetch_or(&self, value: bool, ordering: Ordering) -> bool;
    fn fetch_xor(&self, value: bool, ordering: Ordering) -> bool;
}

/// Marks types that can be used like integer atomics.
pub trait IntAtomic: Atomic {
    fn fetch_add(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
    fn fetch_and(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
    fn fetch_max(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
    fn fetch_min(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
    fn fetch_nand(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
    fn fetch_or(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
    fn fetch_sub(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
    fn fetch_xor(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
}

macro_rules! simple_atomic_impl {
    ($atomic_type:ty => $value_type:ty) => {
        impl $crate::compat::SimpleAtomic for $atomic_type {
            type Value = $value_type;

            fn load(&self, ordering: Ordering) -> Self::Value {
                self.load(ordering)
            }

            fn store(&self, value: Self::Value, ordering: Ordering) {
                self.store(value, ordering)
            }
        }
    };
}

macro_rules! atomic_impl {
    ($atomic_type:ty => $value_type:ty) => {
        $crate::compat::simple_atomic_impl!($atomic_type => $value_type);

        impl $crate::compat::Atomic for $atomic_type {
            // fn compare_and_swap(&self, current: Self::Value, new: Self::Value, ordering: Ordering) -> Self::Value {
            //     self.compare_and_swap(current, new, ordering)
            // }

            fn compare_exchange(&self, current: Self::Value, new: Self::Value, success: Ordering, failure: Ordering) -> Result<Self::Value, Self::Value> {
                self.compare_exchange(current, new, success, failure)
            }

            fn compare_exchange_weak(&self, current: Self::Value, new: Self::Value, success: Ordering, failure: Ordering) -> Result<Self::Value, Self::Value> {
                self.compare_exchange_weak(current, new, success, failure)
            }

            fn fetch_update<F>(&self, set_ordering: Ordering, fetch_ordering: Ordering, f: F) -> Result<Self::Value, Self::Value>
            where
                F: FnMut(Self::Value) -> Option<Self::Value>
            {
                self.fetch_update(set_ordering, fetch_ordering, f)
            }

            fn swap(&self, value: Self::Value, ordering: Ordering) -> Self::Value {
                self.swap(value, ordering)
            }
        }
    };
}

macro_rules! bool_atomic_impl {
    ($atomic_type:ty) => {
        $crate::compat::atomic_impl!($atomic_type => bool);

        impl $crate::compat::BoolAtomic for $atomic_type {
            fn fetch_and(&self, value: bool, ordering: Ordering) -> bool {
                self.fetch_and(value, ordering)
            }

            fn fetch_nand(&self, value: bool, ordering: Ordering) -> bool {
                self.fetch_nand(value, ordering)
            }

            // fn fetch_not(&self, ordering: Ordering) -> bool {
            //     self.fetch_not(ordering)
            // }

            fn fetch_or(&self, value: bool, ordering: Ordering) -> bool {
                self.fetch_or(value, ordering)
            }

            fn fetch_xor(&self, value: bool, ordering: Ordering) -> bool {
                self.fetch_xor(value, ordering)
            }
        }
    };
}

macro_rules! int_atomic_impl {
    ($atomic_type:ty => $value_type:ty) => {
        $crate::compat::atomic_impl!($atomic_type => $value_type);

        impl $crate::compat::IntAtomic for $atomic_type {
            fn fetch_add(&self, value: Self::Value, ordering: Ordering) -> Self::Value {
                self.fetch_add(value, ordering)
            }

            fn fetch_and(&self, value: Self::Value, ordering: Ordering) -> Self::Value {
                self.fetch_and(value, ordering)
            }

            fn fetch_max(&self, value: Self::Value, ordering: Ordering) -> Self::Value {
                self.fetch_max(value, ordering)
            }

            fn fetch_min(&self, value: Self::Value, ordering: Ordering) -> Self::Value {
                self.fetch_min(value, ordering)
            }

            fn fetch_nand(&self, value: Self::Value, ordering: Ordering) -> Self::Value {
                self.fetch_nand(value, ordering)
            }

            fn fetch_or(&self, value: Self::Value, ordering: Ordering) -> Self::Value {
                self.fetch_or(value, ordering)
            }

            fn fetch_sub(&self, value: Self::Value, ordering: Ordering) -> Self::Value {
                self.fetch_sub(value, ordering)
            }

            fn fetch_xor(&self, value: Self::Value, ordering: Ordering) -> Self::Value {
                self.fetch_xor(value, ordering)
            }
        }
    };
}

pub(crate) use {simple_atomic_impl, atomic_impl, bool_atomic_impl, int_atomic_impl};