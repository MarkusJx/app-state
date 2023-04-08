use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;

static STATE: Mutex<Option<HashMap<TypeId, Box<dyn Any + Send>>>> = Mutex::new(None);

fn insert_state<T: 'static + Clone + Send>(state: T) {
    let mut guard = STATE.lock().unwrap();
    guard
        .get_or_insert(HashMap::new())
        .insert(TypeId::of::<T>(), Box::new(state));
}

fn find_state<T: 'static + Clone>() -> Result<T, Box<dyn Error>> {
    let state = STATE.lock().unwrap();
    Ok(state
        .as_ref()
        .ok_or("The state store has not yet been initialized")?
        .clone()
        .get(&TypeId::of::<T>())
        .ok_or("Could not find requested state")?
        .downcast_ref::<T>()
        .ok_or("Could not cast to requested state")?
        .clone())
}

fn find_state_unwrap<T: 'static + Clone>() -> T {
    match find_state::<T>() {
        Ok(state) => state,
        Err(err) => panic!("{}", err),
    }
}

pub mod app_state;
pub mod mut_app_state_lock;
pub mod mutable_app_state;
pub mod traits;
