use core::fmt::Debug;
pub use core::sync::atomic::Ordering;

/// Extension trait for `Ordering`.
pub trait OrderingExt : Clone + Debug {
    /// Converts to the appropriate ordering for a load operation.
    /// 
    /// # Examples
    /// ```
    /// # use atomiq::{Ordering, OrderingExt};
    /// 
    /// let ordering = Ordering::AcqRel;
    /// assert_eq!(ordering.for_load(), Ordering::Acquire);
    /// ```
    /// 
    /// # Panics
    /// Panics if the ordering is `Release` or happens to be unknown.
    fn for_load(self) -> Self;
    
    /// Converts to the appropriate ordering for a store operation.
    ///
    /// # Examples
    /// ```
    /// # use atomiq::{Ordering, OrderingExt};
    ///
    /// let ordering = Ordering::AcqRel;
    /// assert_eq!(ordering.for_store(), Ordering::Release);
    /// ```
    ///
    /// # Panics
    /// Panics if the ordering is `Acquire` or happens to be unknown.
    fn for_store(self) -> Self;
}

impl OrderingExt for Ordering {
    fn for_load(self) -> Self {
        match self {
            Ordering::Relaxed => Ordering::Relaxed,
            Ordering::Acquire => Ordering::Acquire,
            Ordering::Release => panic!("there is no such thing as a release load"),
            Ordering::AcqRel => Ordering::Acquire,
            Ordering::SeqCst => Ordering::SeqCst,
            _ => panic!("unknown ordering: {:?}", self),
        }
    }

    fn for_store(self) -> Self {
        match self {
            Ordering::Relaxed => Ordering::Relaxed,
            Ordering::Acquire => panic!("there is no such thing as an acquire store"),
            Ordering::Release => Ordering::Release,
            Ordering::AcqRel => Ordering::Release,
            Ordering::SeqCst => Ordering::SeqCst,
            _ => panic!("Unknown ordering: {:?}", self),
        }
    }
}