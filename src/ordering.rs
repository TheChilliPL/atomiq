use core::fmt::Debug;
pub use core::sync::atomic::Ordering;

pub trait OrderingExt : Clone + Debug {
    /// Converts to the appropriate ordering for a load operation.
    fn for_load(self) -> Self;
    
    /// Converts to the appropriate ordering for a store operation.
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