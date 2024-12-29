use cfg_if::cfg_if;

/// Initializes the model for testing.
/// 
/// If there is no model to initialize, this function does nothing.
/// 
/// # Example
/// ```
/// use atomiq::try_init_model;
/// 
/// try_init_model(|| {
///    // Perform atomic operations here.
/// });
/// ```
#[inline]
pub fn try_init_model<F>(f: F)
where
    F: Fn() + Sync + Send + 'static
{
    #[cfg(feature = "loom")]
    loom::model(f);
    #[cfg(not(feature = "loom"))]
    f();
}