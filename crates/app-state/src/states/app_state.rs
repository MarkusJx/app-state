use crate::states::traits::CreateAppState;
use crate::AppStateTrait;
use std::ops::Deref;
use std::sync::Arc;

/// A thread-safe, read-only state container.
///
/// # Examples
/// ```rust
/// use app_state::{AppState, AppStateTrait, stateful};
///
/// struct MyState {
///   counter: u32,
/// }
///
/// #[stateful]
/// fn func(state: AppState<MyState>) {
///   let state = state.get_ref();
///   println!("Counter: {}", state.counter);
/// }
/// ```
pub struct AppState<T: ?Sized>(Arc<T>);

impl<T: 'static + Send> CreateAppState<T> for AppState<T> {
    fn new(state: T) -> AppState<T> {
        AppState(Arc::new(state))
    }
}

impl<T: 'static + Send + Sync> AppStateTrait<T, AppState<T>> for AppState<T> {}

impl<T: ?Sized> AppState<T> {
    /// Returns reference to inner `T`.
    pub fn get_ref(&self) -> &T {
        &self.0
    }

    /// Unwraps to the internal `Arc<T>`
    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

impl<T: ?Sized> Deref for AppState<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Arc<T> {
        &self.0
    }
}

impl<T: ?Sized> Clone for AppState<T> {
    fn clone(&self) -> AppState<T> {
        AppState(Arc::clone(&self.0))
    }
}

impl<T: ?Sized> From<Arc<T>> for AppState<T> {
    fn from(arc: Arc<T>) -> Self {
        AppState(arc)
    }
}
