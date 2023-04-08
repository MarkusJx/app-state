use crate::states::traits::CreateAppState;
use crate::{AppStateTrait, MutAppStateLock};
use std::ops::Deref;
use std::sync::{Arc, Mutex};

/// A mutable app state.
///
/// # Examples
/// ```rust
/// use app_state::{MutAppState, AppStateTrait, stateful};
///
/// struct MyState {
///   counter: u32,
/// }
///
/// #[stateful]
/// fn func(state: MutAppState<MyState>) {
///   let mut state = state.get_mut();
///   state.counter += 1;
/// }
/// ```
pub struct MutAppState<T: ?Sized>(Arc<Mutex<T>>);

impl<T: 'static + Send> MutAppState<T> {
    /// Returns reference to inner `T`.
    pub fn get_mut(&self) -> MutAppStateLock<T> {
        MutAppStateLock::new(&self)
    }
}

impl<T: 'static + Send> CreateAppState<T> for MutAppState<T> {
    fn new(state: T) -> MutAppState<T> {
        MutAppState(Arc::new(Mutex::new(state)))
    }
}

impl<T: 'static + Send> AppStateTrait<T, MutAppState<T>> for MutAppState<T> {}

impl<T: ?Sized> MutAppState<T> {
    /// Unwraps to the internal `Arc<T>`
    pub fn into_inner(self) -> Arc<Mutex<T>> {
        self.0
    }
}

impl<T: ?Sized> Deref for MutAppState<T> {
    type Target = Arc<Mutex<T>>;

    fn deref(&self) -> &Arc<Mutex<T>> {
        &self.0
    }
}

impl<T: ?Sized> Clone for MutAppState<T> {
    fn clone(&self) -> MutAppState<T> {
        MutAppState(Arc::clone(&self.0))
    }
}

impl<T: ?Sized> From<Arc<Mutex<T>>> for MutAppState<T> {
    fn from(arc: Arc<Mutex<T>>) -> Self {
        MutAppState(arc)
    }
}
