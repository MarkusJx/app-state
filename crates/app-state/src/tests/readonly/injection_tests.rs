use crate::tests::util::StateTrait;
use crate::{create_creatable_state, create_state, stateful, AppState, AppStateTrait};

struct NonExistentState {}

#[stateful]
fn check_state<T: StateTrait>(state: AppState<T>) {
    assert_eq!(state.get_name(), "Hello");
}

#[stateful(init(state))]
fn init_and_check_state<T: StateTrait + Default>(state: AppState<T>) {
    assert_eq!(state.get_name(), "Hello");
}

#[stateful]
fn check_non_existent_state(_state: AppState<NonExistentState>) {}

#[test]
fn test_get_injected_state() {
    create_state!(AppState);
    check_state::<State>();
}

#[test]
#[should_panic]
fn test_get_non_existent_state() {
    create_state!(AppState);
    check_non_existent_state();
}

#[test]
fn test_init_default_state() {
    create_creatable_state!();
    init_and_check_state::<State>();
}
