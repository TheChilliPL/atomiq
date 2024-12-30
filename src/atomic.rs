use crate::prelude::*;

/// An atomic value.
#[derive(Debug)]
pub struct Atomic<T: Atomizable>(pub(crate) <T::Atom as Atom>::Provider);

impl<T: Atomizable> Default for Atomic<T> {
    fn default() -> Self {
        Atomic(<T::Atom as Atom>::Provider::default())
    }
}

impl<T: Atomizable> From<T> for Atomic<T> {
    fn from(value: T) -> Self {
        Atomic(<T::Atom as Atom>::Provider::from(value.pack()))
    }
}

impl<T: Atomizable> Atomic<T> {
    /// Loads a value with the given ordering.
    pub fn load(&self, ordering: Ordering) -> T {
        T::unpack(Atom::load(&self.0, ordering))
    }
    
    /// Stores a value with the given ordering.
    pub fn store(&self, value: T, ordering: Ordering) {
        Atom::store(&self.0, value.pack(), ordering)
    }
    
    /// Swaps a value with the given ordering.
    pub fn swap(&self, value: T, ordering: Ordering) -> T {
        T::unpack(Atom::swap(&self.0, value.pack(), ordering))
    }
    
    /// Stores a value if the current value is equal to the given value.
    /// 
    /// The return value indicates whether the store was successful and contains
    /// the previous value.
    pub fn compare_exchange(&self, current: T, new: T, success: Ordering, failure: Ordering) -> Result<T, T> {
        Atom::compare_exchange(&self.0, current.pack(), new.pack(), success, failure)
            .map(|value| T::unpack(value))
            .map_err(|value| T::unpack(value))
    }
    
    /// Stores a value if the current value is equal to the given value.
    /// 
    /// The return value indicates whether the store was successful and contains
    /// the previous value.
    /// 
    /// This weak variant might fail even when the value is equal, but it may be
    /// more efficient on some platforms.
    pub fn compare_exchange_weak(&self, current: T, new: T, success: Ordering, failure: Ordering) -> Result<T, T> {
        Atom::compare_exchange_weak(&self.0, current.pack(), new.pack(), success, failure)
            .map(|value| T::unpack(value))
            .map_err(|value| T::unpack(value))
    }
    
    /// Fetches the value, applies a function to it, and optionally stores the result.
    /// 
    /// The return value represents whether the value was updated and contains the
    /// previous value.
    /// 
    /// If value changes between the fetch and store, the function is retried.
    /// 
    /// This method is _not_ provided by the hardware, but implemented by [compare_exchange_weak]
    /// and suffers some drawbacks.
    pub fn fetch_update<F>(&self, set_ordering: Ordering, get_ordering: Ordering, mut f: F) -> Result<T, T>
    where
        F: FnMut(T) -> Option<T>,
    {
        Atom::fetch_update(&self.0, set_ordering, get_ordering, |value| f(T::unpack(value)).map(T::pack))
            .map(|value| T::unpack(value))
            .map_err(|value| T::unpack(value))
    }
}

impl<T: BitAtomizable> Atomic<T> {
    /// Fetches the value, applies a bitwise AND operation to it, and stores the result.
    pub fn fetch_and(&self, value: T, ordering: Ordering) -> T {
        T::unpack(T::Atom::fetch_and(&self.0, value.pack(), ordering))
    }
    
    /// Fetches the value, applies a bitwise NAND operation to it, and stores the result.
    pub fn fetch_nand(&self, value: T, ordering: Ordering) -> T {
        T::unpack(T::Atom::fetch_nand(&self.0, value.pack(), ordering))
    }
    
    /// Fetches the value, applies a bitwise OR operation to it, and stores the result.
    pub fn fetch_or(&self, value: T, ordering: Ordering) -> T {
        T::unpack(T::Atom::fetch_or(&self.0, value.pack(), ordering))
    }
    
    /// Fetches the value, applies a bitwise XOR operation to it, and stores the result.
    pub fn fetch_xor(&self, value: T, ordering: Ordering) -> T {
        T::unpack(T::Atom::fetch_xor(&self.0, value.pack(), ordering))
    }
}

impl<T: IntAtomizable> Atomic<T> {
    /// Fetches the value, adds another value to it, and stores the result.
    pub fn fetch_add(&self, value: T, ordering: Ordering) -> T {
        T::unpack(T::Atom::fetch_add(&self.0, value.pack(), ordering))
    }

    /// Fetches the value, subtracts another value from it, and stores the result.
    pub fn fetch_sub(&self, value: T, ordering: Ordering) -> T {
        T::unpack(T::Atom::fetch_sub(&self.0, value.pack(), ordering))
    }

    /// Fetches the value, calculates the minimum with another value, and stores the result.
    pub fn fetch_min(&self, value: T, ordering: Ordering) -> T {
        T::unpack(T::Atom::fetch_min(&self.0, value.pack(), ordering))
    }

    /// Fetches the value, calculates the maximum with another value, and stores the result.
    pub fn fetch_max(&self, value: T, ordering: Ordering) -> T {
        T::unpack(T::Atom::fetch_max(&self.0, value.pack(), ordering))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    use crate::try_init_model;

    #[test]
    #[cfg(target_has_atomic = "ptr")]
    fn atomic_usize_test() {
        try_init_model(|| {
            let x = 1usize;

            let atomic = Atomic::from(x);

            assert_eq!(atomic.load(Ordering::Relaxed), 1);

            atomic.store(2, Ordering::Relaxed);

            assert_eq!(atomic.load(Ordering::Relaxed), 2);
        });
    }
}