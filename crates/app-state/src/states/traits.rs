use crate::states::{find_state, find_state_unwrap, insert_state, insert_state_if_not_exists};
use std::error::Error;

pub trait InitAppState {
    fn init_app_state(self);
}

pub trait InitMutAppState {
    fn init_mut_app_state(self);
}

pub trait CreateAppState<T: 'static + Send> {
    fn new(state: T) -> Self;
}

pub trait AppStateTrait<T, U>
where
    T: 'static + Send,
    U: 'static + AppStateTrait<T, U> + CreateAppState<T> + Clone + Send,
{
    /// Initializes the state store with the given state.
    /// If the state store has already been initialized, this will overwrite the existing state.
    ///
    /// # Examples
    /// ```rust
    /// use app_state::{AppState, AppStateTrait};
    ///
    /// struct MyState {
    ///   counter: u32,
    /// }
    ///
    /// fn main() {
    ///   let state = MyState { counter: 0 };
    ///   AppState::init(state);
    /// }
    /// ```
    fn init(state: T) {
        insert_state(U::new(state));
    }

    /// Initializes the state store with the given state.
    /// If the state store has already been initialized, this will do nothing.
    /// This is useful for initializing the state store with a default state.
    /// If you want to overwrite the existing state, use `init` instead.
    ///
    /// # Examples
    /// ```rust
    /// use app_state::{AppState, AppStateTrait};
    ///
    /// struct MyState {
    ///   counter: u32,
    /// }
    ///
    /// fn main() {
    ///   AppState::init_if_not_exists(|| MyState { counter: 0 });
    /// }
    /// ```
    fn init_if_not_exists<F: FnOnce() -> T>(state: F) {
        insert_state_if_not_exists(|| U::new(state()));
    }

    fn get() -> U {
        find_state_unwrap()
    }

    fn try_get() -> Result<U, Box<dyn Error>> {
        find_state()
    }
}
