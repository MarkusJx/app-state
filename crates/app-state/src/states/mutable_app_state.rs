use crate::states::{find_state, insert_state};
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};

/// A mutable app state.
///
/// # Examples
/// ```rust
/// use app_state::{MutAppState, stateful};
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
    fn new(state: T) -> MutAppState<T> {
        MutAppState(Arc::new(Mutex::new(state)))
    }

    pub fn init(state: T) {
        insert_state(MutAppState::new(state));
    }

    pub fn get() -> MutAppState<T> {
        find_state()
    }
}

impl<T: ?Sized> MutAppState<T> {
    /// Returns reference to inner `T`.
    pub fn get_mut(&self) -> MutexGuard<'_, T> {
        self.0.lock().unwrap()
    }

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
