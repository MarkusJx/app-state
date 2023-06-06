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
        #[cfg(feature = "log")]
        log::debug!("Initializing state {}", std::any::type_name::<T>());

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
        insert_state_if_not_exists(|| {
            #[cfg(feature = "log")]
            log::debug!("Initializing state {}", std::any::type_name::<T>());

            U::new(state())
        });
    }

    /// Returns a reference to the state.
    /// If the state store has not been initialized, this will panic.
    fn get() -> U {
        find_state_unwrap()
    }

    /// Returns a reference to the state.
    /// If the state store has not been initialized, this will return `Err`.
    fn try_get() -> Result<U, Box<dyn Error>> {
        find_state()
    }

    /// Returns a reference to the state.
    /// Inserts the supplied value if the state store has not been initialized.
    fn get_or_insert(val: T) -> U {
        insert_state_if_not_exists(|| {
            #[cfg(feature = "log")]
            log::debug!("Initializing state {}", std::any::type_name::<T>());

            U::new(val)
        })
    }

    /// Returns a reference to the state.
    /// Inserts the supplied value if the state store has not been initialized.
    fn get_or_insert_with<F: FnOnce() -> T>(f: F) -> U {
        insert_state_if_not_exists(|| {
            #[cfg(feature = "log")]
            log::debug!("Initializing state {}", std::any::type_name::<T>());

            U::new(f())
        })
    }

    /// Returns a reference to the state.
    /// Inserts the default value of `T` if the state store has not been initialized.
    fn get_or_insert_default() -> U
    where
        T: Default,
    {
        insert_state_if_not_exists(|| {
            #[cfg(feature = "log")]
            log::debug!("Initializing state {}", std::any::type_name::<T>());

            U::new(T::default())
        })
    }
}
