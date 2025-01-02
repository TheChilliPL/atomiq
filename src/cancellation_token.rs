use crate::prelude::*;

/// A token that can be used to cancel an async operation.
///
/// Passed by reference to any async operation that should be cancellable.
#[derive(Debug, Default)]
#[repr(transparent)]
pub struct CancellationToken(Atomic<bool>);

impl CancellationToken {
    /// Creates a new cancellation token.
    pub fn new() -> Self {
        Self::default()
    }

    /// Cancels the operation associated with the token.
    /// 
    /// If already cancelled, does nothing.
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }
    
    /// Cancels the operation associated with the token and returns `true` if the operation was
    /// already cancelled.
    pub fn fetch_cancel(&self) -> bool {
        self.0.swap(true, Ordering::Relaxed)
    }

    /// Returns `true` if the token has been cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancellation_token() {
        let token = CancellationToken::new();
        assert!(!token.is_cancelled());

        token.cancel();
        assert!(token.is_cancelled());
    }
}