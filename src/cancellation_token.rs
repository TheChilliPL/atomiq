use core::ops::Deref;
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
}

trait Cancel {
    /// Cancels the operation associated with the token.
    ///
    /// If already cancelled, does nothing.
    fn cancel(&self);
    
    /// Cancels the operation associated with the token and returns `true` if the operation was
    /// already cancelled.
    /// 
    fn fetch_cancel(&self) -> bool;
    /// Returns `true` if the token has been cancelled.
    fn is_cancelled(&self) -> bool;
}

impl Cancel for CancellationToken {
    fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }
    
    fn fetch_cancel(&self) -> bool {
        self.0.swap(true, Ordering::Relaxed)
    }
    
    fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

impl <T: Deref<Target = CancellationToken>> Cancel for Option<T> {
    /// Tries to cancel the operation associated with the token.
    /// 
    /// If already cancelled, does nothing.
    /// If the token is `None`, does nothing.
    fn cancel(&self) {
        if let Some(token) = self {
            token.cancel();
        }
    }
    
    /// Cancels the operation associated with the token and returns `true` if the operation was
    /// already cancelled.
    /// 
    /// If the token is `None`, always returns `false`.
    fn fetch_cancel(&self) -> bool {
        if let Some(token) = self {
            token.fetch_cancel()
        } else {
            false
        }
    }

    /// Returns `true` if the token has been cancelled.
    /// 
    /// If the token is `None`, always returns `false`.
    fn is_cancelled(&self) -> bool {
        if let Some(token) = self {
            token.is_cancelled()
        } else {
            false
        }
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
    
    #[test]
    fn test_option_cancellation_token_some() {
        let token = CancellationToken::new();
        
        let opt_token = Some(&token);
        
        assert!(!opt_token.is_cancelled());
        assert!(!opt_token.fetch_cancel());
        assert!(opt_token.is_cancelled());
        assert!(opt_token.fetch_cancel());
    }
    
    #[test]
    fn test_option_cancellation_token_none() {
        let opt_token: Option<&CancellationToken> = None;
        
        assert!(!opt_token.is_cancelled());
        assert!(!opt_token.fetch_cancel());
        assert!(!opt_token.is_cancelled());
        assert!(!opt_token.fetch_cancel());
    }
}