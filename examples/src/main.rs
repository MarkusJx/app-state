use app_state::{
    init_default_mut_state, init_default_state, stateful, AppState, AppStateTrait, InitAppState,
    InitMutAppState, MutAppState, MutAppStateLock,
};
use std::ops::Deref;

#[derive(InitMutAppState)]
struct MutState {
    name: String,
}

#[derive(InitAppState)]
struct State {
    name: String,
}

#[init_default_state]
#[derive(Default, Debug)]
struct State2 {
    _name: String,
}

#[init_default_mut_state]
#[derive(Default, Debug)]
struct MutState2 {
    _name: String,
}

struct CreatableState {
    name: String,
}

impl Default for CreatableState {
    fn default() -> Self {
        Self {
            name: "Hello".to_string(),
        }
    }
}

#[stateful]
fn change_name(state: MutAppState<MutState>) {
    state.get_mut().name = "Changed".to_string();
}

#[stateful]
fn check_mut_state(state: MutAppState<MutState>) {
    assert_eq!(state.get_mut().name, "Changed".to_string());
}

#[stateful]
fn with_lock(mut state: MutAppStateLock<MutState>) {
    state.name = "Changed1".to_string();
    assert_eq!(state.name, "Changed1".to_string());
}

#[stateful]
fn check_state(state: AppState<State>) {
    assert_eq!(state.name, "Hello".to_string());
}

#[stateful(default(state))]
fn check_creatable_state(state: AppState<CreatableState>) {
    assert_eq!(state.name, "Hello".to_string());
}

fn main() {
    let state2 = AppState::<State2>::get();
    println!("{:?}", state2.get_ref());
    let mut_state2 = MutAppState::<MutState2>::get();
    println!("{:?}", mut_state2.get_mut().deref());

    MutState {
        name: "Hello".to_string(),
    }
    .init_mut_app_state();
    State {
        name: "Hello".to_string(),
    }
    .init_app_state();

    change_name();
    check_mut_state();
    check_state();
    with_lock();
    check_creatable_state();
}
