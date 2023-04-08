use crate::{AppState, AppStateTrait, MutAppState};

struct State {
    name: String,
}

struct MutState {
    name: String,
}

fn create_state() {
    AppState::init(State {
        name: "Hello".to_string(),
    });
}

#[test]
fn test_get_state() {
    create_state();

    let state = AppState::<State>::get();
    assert_eq!(state.name, "Hello".to_string());
}

#[test]
fn test_try_get_state() {
    create_state();

    let state = AppState::<State>::try_get();
    assert!(state.is_ok());
    assert_eq!(state.unwrap().name, "Hello".to_string());
}

#[test]
#[should_panic]
fn test_get_non_existent_state() {
    create_state();
    AppState::<MutState>::get();
}

#[test]
fn test_try_get_non_existent_state() {
    create_state();
    let state = AppState::<MutState>::try_get();
    assert!(state.is_err());
}

fn create_mutable_state() {
    MutAppState::init(MutState {
        name: "Hello".to_string(),
    });
}

#[test]
fn test_get_mutable_state() {
    create_mutable_state();
    let state = MutAppState::<MutState>::get();
    assert_eq!(state.get_mut().name, "Hello".to_string());
}

#[test]
fn test_try_get_mutable_state() {
    create_mutable_state();
    let state = MutAppState::<MutState>::try_get();
    assert!(state.is_ok());
    assert_eq!(state.unwrap().get_mut().name, "Hello".to_string());
}

#[test]
#[should_panic]
fn test_get_non_existent_mutable_state() {
    create_mutable_state();
    MutAppState::<State>::get();
}

#[test]
fn test_try_get_non_existent_mutable_state() {
    create_mutable_state();
    let state = MutAppState::<State>::try_get();
    assert!(state.is_err());
}

#[test]
fn test_change_state() {
    create_mutable_state();

    let state = MutAppState::<MutState>::get();
    let mut state = state.get_mut();

    assert_eq!(state.name, "Hello".to_string());
    state.name = "Changed".to_string();

    assert_eq!(state.name, "Changed".to_string());
    drop(state);

    let state = MutAppState::<MutState>::get();
    assert_eq!(state.get_mut().name, "Changed".to_string());
}
