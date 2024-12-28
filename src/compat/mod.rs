//! Atomic compatibility layers.

/// Compatibility layer for `core::sync::atomic`.
#[cfg(feature = "compat_core")]
pub mod core;
/// Compatibility layer for `loom::sync::atomic`.
#[cfg(feature = "compat_loom")]
pub mod loom;

use crate::prelude::*;

/// Marks types that can be used like simple atomics.
///
/// Simple atomics only have to support [load](SimpleAtomic::load) and [store](SimpleAtomic::store) 
/// atomic operations.
pub trait SimpleAtomic: From<Self::Value> + Default {
    /// The type of the value stored in the atomic.
    type Value;

    /// Creates a new atomic from a value.
    fn new(value: Self::Value) -> Self {
        Self::from(value)
    }
    /// Loads the value from the atomic with the given ordering.
    /// 
    /// See more in [`core`].
    fn load(&self, ordering: Ordering) -> Self::Value;
    /// Stores the value in the atomic with the given ordering.
    /// 
    /// See more in [`core::sync::atomic::AtomicUsize::store`].
    fn store(&self, value: Self::Value, ordering: Ordering);
}

/// Marks types that can be used like atomics.
pub trait Atomic: SimpleAtomic {
    // Deprecated
    // fn compare_and_swap(&self, current: Self::Value, new: Self::Value, ordering: Ordering) -> Self::Value;
    /// Compares the value in the atomic with `current` and if equal, replaces it with `new`.
    /// 
    /// Returns the previous value.
    /// 
    /// See more in [`core::sync::atomic::AtomicIsize::compare_exchange`].
    fn compare_exchange(&self, current: Self::Value, new: Self::Value, success: Ordering, failure: Ordering) -> Result<Self::Value, Self::Value>;
    /// Compares the value in the atomic with `current` and if equal, replaces it with `new`.
    /// 
    /// Returns the previous value.
    /// 
    /// Allowed to fail spuriously, which can result in more efficient code.
    /// 
    /// See more in [`core::sync::atomic::AtomicIsize::compare_exchange_weak`].
    fn compare_exchange_weak(&self, current: Self::Value, new: Self::Value, success: Ordering, failure: Ordering) -> Result<Self::Value, Self::Value>;
    /// Updates the value in the atomic using a closure.
    /// 
    /// See more in [`core::sync::atomic::AtomicIsize::fetch_update`].
    fn fetch_update<F>(&self, set_ordering: Ordering, fetch_ordering: Ordering, f: F) -> Result<Self::Value, Self::Value>
    where
        F: FnMut(Self::Value) -> Option<Self::Value>;
    /// Swaps the value in the atomic with `value`.
    /// 
    /// Returns the previous value.
    /// 
    /// See more in [`core::sync::atomic::AtomicIsize::swap`].
    fn swap(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
}

/// Marks types that can be used like boolean atomics.
pub trait BoolAtomic: Atomic<Value = bool> {
    /// Atomically sets the current value to the logical AND of the current value and `value`
    /// with the given ordering.
    /// 
    /// Returns the previous value.
    /// 
    /// See more in [`core::sync::atomic::AtomicBool::fetch_and`].
    fn fetch_and(&self, value: bool, ordering: Ordering) -> bool;
    /// Atomically sets the current value to the logical NAND of the current value and `value`
    /// with the given ordering.
    /// 
    /// Returns the previous value.
    /// 
    /// See more in [`core::sync::atomic::AtomicBool::fetch_nand`].
    fn fetch_nand(&self, value: bool, ordering: Ordering) -> bool;
    // Loom does not support this yet, as it was only recently stabilized.
    // https://github.com/tokio-rs/loom/issues/371
    // fn fetch_not(&self, ordering: Ordering) -> bool;
    /// Atomically sets the current value to the logical OR of the current value and `value`
    /// with the given ordering.
    /// 
    /// Returns the previous value.
    /// 
    /// See more in [`core::sync::atomic::AtomicBool::fetch_or`].
    fn fetch_or(&self, value: bool, ordering: Ordering) -> bool;
    /// Atomically sets the current value to the logical XOR of the current value and `value`
    /// with the given ordering.
    /// 
    /// Returns the previous value.
    /// 
    /// See more in [`core::sync::atomic::AtomicBool::fetch_xor`].
    fn fetch_xor(&self, value: bool, ordering: Ordering) -> bool;
}

/// Marks types that can be used like integer atomics.
pub trait IntAtomic: Atomic {
    /// Atomically adds `value` to the current value with the given ordering.
    /// 
    /// Returns the previous value.
    /// 
    /// See more in [`core::sync::atomic::AtomicIsize::fetch_add`].
    fn fetch_add(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
    /// Atomically sets the current value to the bitwise AND of the current value and `value`
    /// with the given ordering.
    /// 
    /// Returns the previous value.
    /// 
    /// See more in [`core::sync::atomic::AtomicIsize::fetch_and`].
    fn fetch_and(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
    /// Atomically sets the current value to the maximum of the current value and `value`
    /// with the given ordering.
    /// 
    /// Returns the previous value.
    /// 
    /// See more in [`core::sync::atomic::AtomicIsize::fetch_max`].
    fn fetch_max(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
    /// Atomically sets the current value to the minimum of the current value and `value`
    /// with the given ordering.
    /// 
    /// Returns the previous value.
    /// 
    /// See more in [`core::sync::atomic::AtomicIsize::fetch_min`].
    fn fetch_min(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
    /// Atomically sets the current value to the bitwise NAND of the current value and `value`
    /// with the given ordering.
    /// 
    /// Returns the previous value.
    /// 
    /// See more in [`core::sync::atomic::AtomicIsize::fetch_nand`].
    fn fetch_nand(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
    /// Atomically sets the current value to the bitwise OR of the current value and `value`
    /// with the given ordering.
    /// 
    /// Returns the previous value.
    /// 
    /// See more in [`core::sync::atomic::AtomicIsize::fetch_or`].
    fn fetch_or(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
    /// Atomically subtracts `value` from the current value with the given ordering.
    ///
    /// Returns the previous value.
    /// 
    /// See more in [`core::sync::atomic::AtomicIsize::fetch_sub`].
    fn fetch_sub(&self, value: Self::Value, ordering: Ordering) -> Self::Value;
    /// Atomically sets the current value to the bitwise XOR of the current value and `value`
    /// with the given ordering.
    /// 
    /// Returns the previous value.
    /// 
    /// See more in [`core::sync::atomic::AtomicIsize::fetch_xor`].
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