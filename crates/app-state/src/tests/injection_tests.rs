use crate::{AppState, AppStateTrait, MutAppState, MutAppStateLock};
use app_state_macros::stateful;

struct State {
    name: String,
}

struct ReadOnlyMutState {
    name: String,
}

struct MutState {
    name: String,
}

struct LockState {
    name: String,
}

struct LockMutState {
    name: String,
}

fn create_state() {
    AppState::init(State {
        name: "Hello".to_string(),
    });
}

fn create_lock_state() {
    MutAppState::init(LockState {
        name: "Hello".to_string(),
    });

    MutAppState::init(LockMutState {
        name: "Hello".to_string(),
    });
}

fn create_mutable_state() {
    MutAppState::init(MutState {
        name: "Hello".to_string(),
    });

    MutAppState::init(ReadOnlyMutState {
        name: "Hello".to_string(),
    });
}

#[stateful]
fn check_state(state: AppState<State>) {
    assert_eq!(state.name, "Hello".to_string());
}

#[stateful]
fn check_non_existent_state(_state: AppState<MutState>) {}

#[stateful]
fn check_mut_state(state: MutAppState<ReadOnlyMutState>) {
    assert_eq!(state.get_mut().name, "Hello".to_string());
}

#[stateful]
fn change_name(state: MutAppState<MutState>) {
    state.get_mut().name = "Changed".to_string();
}

#[stateful]
fn check_mut_state_changed(state: MutAppState<MutState>) {
    assert_eq!(state.get_mut().name, "Changed".to_string());
}

#[stateful]
fn check_mut_non_existent_state(_state: MutAppState<State>) {}

#[stateful]
fn check_mut_state_with_lock(state: MutAppStateLock<LockState>) {
    assert_eq!(state.name, "Hello".to_string());
}

#[stateful]
fn change_name_with_lock(mut state: MutAppStateLock<LockMutState>) {
    state.name = "Changed".to_string();
}

#[stateful]
fn check_mut_state_changed_with_lock(state: MutAppStateLock<LockMutState>) {
    assert_eq!(state.name, "Changed".to_string());
}

#[stateful]
fn check_mut_non_existent_state_with_lock(_state: MutAppStateLock<State>) {}

#[test]
fn test_get_injected_state() {
    create_state();
    check_state();
}

#[test]
#[should_panic]
fn test_get_non_existent_state() {
    create_state();
    check_non_existent_state();
}

#[test]
fn test_get_mutable_state() {
    create_mutable_state();
    check_mut_state();
}

#[test]
fn test_change_mutable_state() {
    create_mutable_state();
    change_name();
    check_mut_state_changed();
}

#[test]
#[should_panic]
fn test_get_non_existent_mutable_state() {
    create_mutable_state();
    check_mut_non_existent_state();
}

#[test]
fn test_get_mutable_state_with_lock() {
    create_lock_state();
    check_mut_state_with_lock();
}

#[test]
fn test_change_mutable_state_with_lock() {
    create_lock_state();
    change_name_with_lock();
    check_mut_state_changed_with_lock();
}

#[test]
#[should_panic]
fn test_get_non_existent_mutable_state_with_lock() {
    create_lock_state();
    check_mut_non_existent_state_with_lock();
}
