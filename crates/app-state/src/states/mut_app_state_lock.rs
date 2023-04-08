use crate::MutAppState;
use std::ops::{Deref, DerefMut};
use std::sync::MutexGuard;

/// The lock guard for a mutable app state.
/// This is a wrapper around `MutexGuard`.
/// When this guard is dropped, the lock will be released.
/// As this locks the state, no other thread can access the state until this guard is dropped.
///
/// # Examples
/// ```rust
/// use app_state::{MutAppState, MutAppStateLock, AppStateTrait, stateful};
///
/// struct MyState {
///   counter: u32,
/// }
///
/// #[stateful]
/// fn func(mut state: MutAppStateLock<MyState>) {
///   state.counter += 1;
/// }
/// ```
pub struct MutAppStateLock<'a, T: ?Sized>(MutexGuard<'a, T>);

impl<'a, T: 'static + Send> MutAppStateLock<'a, T> {
    pub fn new(inner: &'a MutAppState<T>) -> MutAppStateLock<'a, T> {
        MutAppStateLock(inner.lock().unwrap())
    }
}

impl<'a, T: ?Sized> MutAppStateLock<'a, T> {
    /// Returns reference to inner `T`.
    pub fn get_ref(&self) -> &MutexGuard<'a, T> {
        &self.0
    }

    /// Unwraps to the internal `Arc<T>`
    pub fn into_inner(self) -> MutexGuard<'a, T> {
        self.0
    }
}

impl<'a, T: ?Sized> Deref for MutAppStateLock<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<'a, T: ?Sized> DerefMut for MutAppStateLock<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
