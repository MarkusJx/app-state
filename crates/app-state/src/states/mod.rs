use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Mutex;

static STATE: Mutex<Option<HashMap<TypeId, Box<dyn Any + Send>>>> = Mutex::new(None);

fn insert_state<T: 'static + Clone + Send>(state: T) {
    let mut st = STATE.lock().unwrap();
    if st.is_none() {
        *st = Some(HashMap::new());
    }

    let st = st.as_mut().unwrap();
    st.insert(TypeId::of::<T>(), Box::new(state));
}

fn find_state<T: 'static + Clone>() -> T {
    let state = STATE.lock().unwrap();
    state
        .as_ref()
        .or_else(|| panic!("The state store has not yet been initialized"))
        .unwrap()
        .clone()
        .get(&TypeId::of::<T>())
        .or_else(|| panic!("Could not find requested state"))
        .unwrap()
        .downcast_ref::<T>()
        .or_else(|| panic!("Could not cast to requested state"))
        .unwrap()
        .clone()
}

pub mod app_state;
pub mod mutable_app_state;
pub mod traits;
