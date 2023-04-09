use crate::tests::util::StateTrait;
use crate::{create_state, AppState, AppStateTrait};

struct NonExistentState {}

#[test]
fn test_get_state() {
    create_state!(AppState);

    let state = AppState::<State>::get();
    assert_eq!(state.name, "Hello".to_string());
}

#[test]
fn test_try_get_state() {
    create_state!(AppState);

    let state = AppState::<State>::try_get();
    assert!(state.is_ok());
    assert_eq!(state.unwrap().name, "Hello".to_string());
}

#[test]
#[should_panic]
fn test_get_non_existent_state() {
    create_state!(AppState);
    AppState::<NonExistentState>::get();
}

#[test]
fn test_try_get_non_existent_state() {
    create_state!(AppState);
    let state = AppState::<NonExistentState>::try_get();
    assert!(state.is_err());
}
