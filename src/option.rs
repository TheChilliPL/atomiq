//! Simple atomic options.
//! 
//! See [`AtomicOption`] for more information.
#![doc()]

use crate::prelude::*;

/// An atomic option.
///
/// This is a lock-free version of `Mutex<Option<T>>`, where `T` is an atomic type.
///
/// Note that if you use [`Ordering::Relaxed`], the `is_some` flag may be set to `true`
/// before the value is set, so this ordering should be avoided. With orderings stronger
/// than `Relaxed`, the value is guaranteed to be set before the `is_some` flag.
/// 
/// # Examples
/// ```
/// use atomiq::prelude::*;
/// use atomiq::option::AtomicOption;
/// # use atomiq::try_init_model;
///
/// # try_init_model(|| {
/// let option: AtomicOption<i32> = AtomicOption::none();
///
/// assert!(option.is_none(Ordering::Acquire));
///
/// option.store_some(42, Ordering::Release);
///
/// assert!(option.is_some(Ordering::Acquire));
/// assert_eq!(option.load(Ordering::Acquire), Some(42));
/// # });
/// ```
#[derive(Debug)]
pub struct AtomicOption<T: Atomizable> {
    is_some: Atomic<bool>,
    value: Atomic<T>,
}

impl<T: Atomizable> From<Option<T>> for AtomicOption<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(value) => Self::some(value),
            None => Self::none(),
        }
    }
}

impl<T: Atomizable> AtomicOption<T> {
    /// Creates a new atomic option with no value.
    pub fn none() -> Self {
        Self {
            is_some: Atomic::from(false),
            value: Atomic::default(),
        }
    }

    /// Creates a new atomic option with a value.
    pub fn some(value: T) -> Self {
        Self {
            is_some: Atomic::from(true),
            value: Atomic::from(value),
        }
    }

    /// Returns whether the option is `Some`.
    pub fn is_some(&self, ordering: Ordering) -> bool {
        self.is_some.load(ordering)
    }

    /// Returns whether the option is `None`.
    pub fn is_none(&self, ordering: Ordering) -> bool {
        !self.is_some(ordering)
    }

    /// Loads the value with the given ordering.
    pub fn load(&self, ordering: Ordering) -> Option<T> {
        if self.is_some(ordering) {
            Some(self.value.load(ordering))
        } else {
            None
        }
    }

    /// Stores a value with the given ordering.
    pub fn store(&self, value: Option<T>, ordering: Ordering) {
        match value {
            Some(value) => {
                // First store the value, then set the flag,
                // so that the value is never read before it's initialized.
                self.value.store(value, ordering);
                self.is_some.store(true, ordering);
            }
            None => {
                self.is_some.store(false, ordering);
            }
        }
    }

    /// Stores `None` with the given ordering.
    pub fn store_none(&self, ordering: Ordering) {
        self.store(None, ordering);
    }

    /// Stores `Some` with the given ordering.
    pub fn store_some(&self, value: T, ordering: Ordering) {
        self.store(Some(value), ordering);
    }

    /// Unwraps the value with the given ordering.
    /// 
    /// # Panics
    /// Panics if the option is `None`.
    pub fn unwrap(&self, ordering: Ordering) -> T {
        self.load(ordering).unwrap()
    }
}

#[cfg(test)]
#[cfg(feature = "loom")]
mod tests {
    use super::*;
    use alloc::vec::Vec;
    use log::{debug, info};
    use loom::thread;
    use crate::option::AtomicOption;
    
    extern crate std;
    use std::sync::atomic::AtomicU32;
    use std::sync::Arc as RealArc;

    fn test_atomic_option_sync(ordering: Ordering) {
        let option: AtomicOption<i32> = AtomicOption::none();

        assert!(option.is_none(ordering.for_load()));
        assert!(!option.is_some(ordering.for_load()));
        assert_eq!(option.load(ordering.for_load()), None);

        option.store_some(42, ordering.for_store());

        assert!(!option.is_none(ordering.for_load()));
        assert!(option.is_some(ordering.for_load()));
        assert_eq!(option.load(ordering.for_load()), Some(42));

        option.store_none(ordering.for_store());

        assert!(option.is_none(ordering.for_load()));
        assert!(!option.is_some(ordering.for_load()));
        assert_eq!(option.load(ordering.for_load()), None);
    }

    fn test_atomic_option_async(ordering: Ordering) {
        let fence = move || {
            if ordering != Ordering::Relaxed {
                fence(ordering);
            }
        };

        // Arc so that we don't have to deal with lifetimes.
        let option: Arc<AtomicOption<i32>> = Arc::new(AtomicOption::none());

        let mut threads = Vec::new();

        threads.push(thread::spawn({
            let option = option.clone();
            move || {
                assert!(option.is_none(ordering.for_load()));
                assert!(!option.is_some(ordering.for_load()));
                assert_eq!(option.load(ordering.for_load()), None);
                debug!("Storing 42");
                fence();

                option.store_some(42, ordering.for_store());

                fence();
                debug!("Stored 42");
                assert!(!option.is_none(ordering.for_load()));
                assert!(option.is_some(ordering.for_load()));
                assert_eq!(option.load(ordering.for_load()), Some(42));
            }
        }));

        threads.push(thread::spawn({
            let option = option.clone();
            move || {
                for _ in 0..3 {
                    fence();
                    debug!("Loading");
                    let res = option.load(ordering.for_load());
                    debug!("Loaded {:?}", res);
                    fence();

                    if let Some(value) = res {
                        assert_eq!(value, 42);
                        break;
                    }
                }
            }
        }));

        for thread in threads {
            thread.join().unwrap();
        }
    }

    #[test]
    fn test_atomic_option_loom_sync() {
        let i = RealArc::new(AtomicU32::new(0));
        loom::model(move || {
            let i = i.fetch_add(1, Ordering::Relaxed) + 1;
            info!("Testing iteration {i}...");
            test_atomic_option_sync(Ordering::Relaxed);
        });
    }

    #[test]
    #[should_panic]
    fn test_atomic_option_loom_async_relaxed() {
        let i = RealArc::new(AtomicU32::new(0));
        loom::model(move || {
            let i = i.fetch_add(1, Ordering::Relaxed) + 1;
            info!("Testing iteration {i}...");
            test_atomic_option_async(Ordering::Relaxed);
        });
    }

    #[test]
    fn test_atomic_option_loom_async_acqrel() {
        let i = RealArc::new(AtomicU32::new(0));
        loom::model(move || {
            let i = i.fetch_add(1, Ordering::Relaxed) + 1;
            info!("Testing iteration {i}...");
            test_atomic_option_async(Ordering::AcqRel);
        });
    }
}
