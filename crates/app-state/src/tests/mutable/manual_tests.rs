use crate::tests::util::StateTrait;
use crate::{create_creatable_state, create_state, AppStateTrait, MutAppState};

struct NonExistentState {}

#[test]
fn test_get_mutable_state() {
    create_state!(MutAppState);
    let state = MutAppState::<State>::get();
    assert_eq!(state.get_mut().name, "Hello".to_string());
}

#[test]
fn test_try_get_mutable_state() {
    create_state!(MutAppState);
    let state = MutAppState::<State>::try_get();
    assert!(state.is_ok());
    assert_eq!(state.unwrap().get_mut().name, "Hello".to_string());
}

#[test]
#[should_panic]
fn test_get_non_existent_mutable_state() {
    create_state!(MutAppState);
    MutAppState::<NonExistentState>::get();
}

#[test]
fn test_try_get_non_existent_mutable_state() {
    create_state!(MutAppState);
    let state = MutAppState::<NonExistentState>::try_get();
    assert!(state.is_err());
}

#[test]
fn test_change_state() {
    create_state!(MutAppState);

    let state = MutAppState::<State>::get();
    let mut state = state.get_mut();

    assert_eq!(state.name, "Hello".to_string());
    state.name = "Changed".to_string();

    assert_eq!(state.name, "Changed".to_string());
    drop(state);

    let state = MutAppState::<State>::get();
    assert_eq!(state.get_mut().name, "Changed".to_string());
}

#[test]
fn test_get_or_insert() {
    create_creatable_state!();

    let state = MutAppState::<State>::get_or_insert(State::default());
    assert_eq!(state.get_mut().name, "Hello".to_string());
}

#[test]
fn test_get_or_insert_with() {
    create_creatable_state!();

    let state = MutAppState::<State>::get_or_insert_with(|| State::default());
    assert_eq!(state.get_mut().name, "Hello".to_string());
}

#[test]
fn test_get_or_insert_default() {
    create_creatable_state!();

    let state = MutAppState::<State>::get_or_insert_default();
    assert_eq!(state.get_mut().name, "Hello".to_string());
}
