use crate::tests::util::StateTrait;
use crate::{
    create_creatable_state, create_state, stateful, AppStateTrait, MutAppState, MutAppStateLock,
};

struct NonExistentState {}

#[stateful]
fn check_mut_state_with_lock<T: StateTrait>(state: MutAppStateLock<T>) {
    assert_eq!(state.get_name(), "Hello");
}

#[stateful]
fn change_name_with_lock<T: StateTrait>(mut state: MutAppStateLock<T>) {
    state.set_name("Changed");
}

#[stateful]
fn check_mut_state_changed_with_lock<T: StateTrait>(state: MutAppStateLock<T>) {
    assert_eq!(state.get_name(), "Changed");
}

#[stateful]
fn check_mut_non_existent_state_with_lock(_state: MutAppStateLock<NonExistentState>) {}

#[stateful(init(state))]
fn init_and_check_state<T: StateTrait + Default>(state: MutAppStateLock<T>) {
    assert_eq!(state.get_name(), "Hello");
}

#[stateful(init(state))]
fn init_check_and_mutate_state<T: StateTrait + Default>(mut state: MutAppStateLock<T>) {
    assert_eq!(state.get_name(), "Hello");
    state.set_name("Changed");
}

#[test]
fn test_get_mutable_state_with_lock() {
    create_state!(MutAppState);
    check_mut_state_with_lock::<State>();
}

#[test]
fn test_change_mutable_state_with_lock() {
    create_state!(MutAppState);
    change_name_with_lock::<State>();
    check_mut_state_changed_with_lock::<State>();
}

#[test]
#[should_panic]
fn test_get_non_existent_mutable_state_with_lock() {
    create_state!(MutAppState);
    check_mut_non_existent_state_with_lock();
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
    check_mut_state_changed_with_lock::<State>();
}
