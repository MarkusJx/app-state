use crate::tests::util::StateTrait;
use crate::{create_creatable_state, create_state, stateful, AppStateTrait, MutAppState};

struct NonExistentState {}

#[stateful]
fn check_mut_state<T: StateTrait>(state: MutAppState<T>) {
    assert_eq!(state.get_mut().get_name(), "Hello");
}

#[stateful]
fn change_name<T: StateTrait>(state: MutAppState<T>) {
    state.get_mut().set_name("Changed");
}

#[stateful]
fn check_mut_state_changed<T: StateTrait>(state: MutAppState<T>) {
    assert_eq!(state.get_mut().get_name(), "Changed");
}

#[stateful]
fn check_mut_non_existent_state(_state: MutAppState<NonExistentState>) {}

#[stateful(init(state))]
fn init_and_check_state<T: StateTrait + Default>(state: MutAppState<T>) {
    assert_eq!(state.get_mut().get_name(), "Hello");
}

#[stateful(init(state))]
fn init_check_and_mutate_state<T: StateTrait + Default>(state: MutAppState<T>) {
    assert_eq!(state.get_mut().get_name(), "Hello");
    state.get_mut().set_name("Changed");
}

#[test]
fn test_get_mutable_state() {
    create_state!(MutAppState);
    check_mut_state::<State>();
}

#[test]
fn test_change_mutable_state() {
    create_state!(MutAppState);
    change_name::<State>();
    check_mut_state_changed::<State>();
}

#[test]
#[should_panic]
fn test_get_non_existent_mutable_state() {
    create_state!(MutAppState);
    check_mut_non_existent_state();
}

#[test]
fn test_init_default_state() {
    create_creatable_state!();
    init_and_check_state::<State>();
}

#[test]
fn test_init_check_and_mutate_state() {
    create_creatable_state!();
    init_check_and_mutate_state::<State>();
    check_mut_state_changed::<State>();
}
