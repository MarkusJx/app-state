use crate::states::{find_state, find_state_unwrap, insert_state};
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
    fn init(state: T) {
        insert_state(U::new(state));
    }

    fn get() -> U {
        find_state_unwrap()
    }

    fn try_get() -> Result<U, Box<dyn Error>> {
        find_state()
    }
}
