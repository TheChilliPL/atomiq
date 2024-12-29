use core::fmt::Debug;
use crate::prelude::*;
use cfg_if::cfg_if;

cfg_if!(
    if #[cfg(feature = "loom")] {
        use loom::sync::atomic as a;
        pub use loom::sync::Arc;
    } else {
        use core::sync::atomic as a;
        #[cfg(feature = "alloc")]
        pub use alloc::sync::Arc;
    }
);
pub use a::fence;

/// A primitive atomizable value.
pub trait Atom: Sized + Clone + Copy + Debug {
    /// The provider of the atomic operations.
    type Provider: From<Self> + Debug + Default;

    #[doc(hidden)]
    fn load(provider: &Self::Provider, ordering: Ordering) -> Self;
    #[doc(hidden)]
    fn store(provider: &Self::Provider, value: Self, ordering: Ordering);
    #[doc(hidden)]
    fn swap(provider: &Self::Provider, value: Self, ordering: Ordering) -> Self;
    #[doc(hidden)]
    fn compare_exchange(
        provider: &Self::Provider,
        current: Self,
        new: Self,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self, Self>;
    #[doc(hidden)]
    fn compare_exchange_weak(
        provider: &Self::Provider,
        current: Self,
        new: Self,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self, Self>;
    #[doc(hidden)]
    fn fetch_update<F>(
        provider: &Self::Provider,
        set_ordering: Ordering,
        fetch_ordering: Ordering,
        f: F,
    ) -> Result<Self, Self>
    where
        F: FnMut(Self) -> Option<Self>;
}

/// A primitive atomizable bit value.
pub trait BitAtom: Atom {
    #[doc(hidden)]
    fn fetch_and(provider: &Self::Provider, value: Self, ordering: Ordering) -> Self;
    #[doc(hidden)]
    fn fetch_nand(provider: &Self::Provider, value: Self, ordering: Ordering) -> Self;
    // Loom does not support this yet, as it was only recently stabilized.
    // https://github.com/tokio-rs/loom/issues/371
    // fn fetch_not(provider: &Self::Provider, ordering: Ordering) -> Self;
    #[doc(hidden)]
    fn fetch_or(provider: &Self::Provider, value: Self, ordering: Ordering) -> Self;
    #[doc(hidden)]
    fn fetch_xor(provider: &Self::Provider, value: Self, ordering: Ordering) -> Self;
}

/// A primitive atomizable integer value.
pub trait IntAtom: Atom {
    #[doc(hidden)]
    fn fetch_add(provider: &Self::Provider, value: Self, ordering: Ordering) -> Self;
    #[doc(hidden)]
    fn fetch_sub(provider: &Self::Provider, value: Self, ordering: Ordering) -> Self;
    #[doc(hidden)]
    fn fetch_min(provider: &Self::Provider, value: Self, ordering: Ordering) -> Self;
    #[doc(hidden)]
    fn fetch_max(provider: &Self::Provider, value: Self, ordering: Ordering) -> Self;
}

macro_rules! or_def {
    ($value:tt or $default:tt) => {
        $value
    };
    (or $default:tt) => {
        $default
    };
}

macro_rules! atom_impl {
    ($atom:ty => $provider:ident $length:literal) => {
        #[cfg(target_has_atomic = $length)]
        use a::$provider;

        #[cfg(target_has_atomic = $length)]
        impl Atom for $atom {
            type Provider = $provider;

            fn load(provider: &$provider, ordering: Ordering) -> Self {
                provider.load(ordering)
            }

            fn store(provider: &$provider, value: Self, ordering: Ordering) {
                provider.store(value, ordering)
            }

            fn swap(provider: &$provider, value: Self, ordering: Ordering) -> Self {
                provider.swap(value, ordering)
            }

            fn compare_exchange(provider: &$provider, current: Self, new: Self, success: Ordering, failure: Ordering) -> Result<Self, Self> {
                provider.compare_exchange(current, new, success, failure)
            }

            fn compare_exchange_weak(provider: &$provider, current: Self, new: Self, success: Ordering, failure: Ordering) -> Result<Self, Self> {
                provider.compare_exchange_weak(current, new, success, failure)
            }

            fn fetch_update<F>(
                provider: &$provider,
                set_ordering: Ordering,
                fetch_ordering: Ordering,
                mut f: F,
            ) -> Result<Self, Self>
            where
                F: FnMut(Self) -> Option<Self>
            {
                provider.fetch_update(set_ordering, fetch_ordering, |value| f(value))
            }
        }
    };
    ($atom:ty => $provider:ident $length:literal bit) => {
        atom_impl!($atom => $provider $length);

        #[cfg(target_has_atomic = $length)]
        impl BitAtom for $atom {
            fn fetch_and(provider: &$provider, value: Self, ordering: Ordering) -> Self {
                provider.fetch_and(value, ordering)
            }

            fn fetch_nand(provider: &$provider, value: Self, ordering: Ordering) -> Self {
                provider.fetch_nand(value, ordering)
            }

            fn fetch_or(provider: &$provider, value: Self, ordering: Ordering) -> Self {
                provider.fetch_or(value, ordering)
            }

            fn fetch_xor(provider: &$provider, value: Self, ordering: Ordering) -> Self {
                provider.fetch_xor(value, ordering)
            }
        }
    };
    ($atom:ty => $provider:ident $length:literal int) => {
        atom_impl!($atom => $provider $length bit);

        #[cfg(target_has_atomic = $length)]
        impl IntAtom for $atom {
            fn fetch_add(provider: &$provider, value: Self, ordering: Ordering) -> Self {
                provider.fetch_add(value, ordering)
            }

            fn fetch_sub(provider: &$provider, value: Self, ordering: Ordering) -> Self {
                provider.fetch_sub(value, ordering)
            }

            fn fetch_min(provider: &$provider, value: Self, ordering: Ordering) -> Self {
                provider.fetch_min(value, ordering)
            }

            fn fetch_max(provider: &$provider, value: Self, ordering: Ordering) -> Self {
                provider.fetch_max(value, ordering)
            }
        }
    };
}

macro_rules! atom_impls {
    ($($atom:ty => $provider:ident $length:literal $($s:ident)?;)+) => {
        $(
            atom_impl!($atom => $provider $length $($s)?);
        )+
    };
}

atom_impls!(
    bool => AtomicBool "8" bit;
    u8 => AtomicU8 "8" int;
    u16 => AtomicU16 "16" int;
    u32 => AtomicU32 "32" int;
    u64 => AtomicU64 "64" int;
    // u128 => AtomicU128 "128" int;
    usize => AtomicUsize "ptr" int;
    i8 => AtomicI8 "8" int;
    i16 => AtomicI16 "16" int;
    i32 => AtomicI32 "32" int;
    i64 => AtomicI64 "64" int;
    // i128 => AtomicI128 "128" int;
    isize => AtomicIsize "ptr" int;
);