use crate::{init_default_state, AppState, AppStateTrait, MutAppState};
use app_state_macros::{init_default_mut_state, stateful};

#[init_default_state]
struct State {
    name: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            name: "Hello".to_string(),
        }
    }
}

#[init_default_mut_state]
struct MutState {
    name: String,
}

impl Default for MutState {
    fn default() -> Self {
        Self {
            name: "Hello".to_string(),
        }
    }
}

#[init_default_state]
#[init_default_mut_state]
struct BothState {
    name: String,
}

impl Default for BothState {
    fn default() -> Self {
        Self {
            name: "Hello".to_string(),
        }
    }
}

#[test]
fn test_get_state() {
    let state = AppState::<State>::get();
    assert_eq!(state.name, "Hello".to_string());
}

#[test]
fn test_inject_state() {
    #[stateful]
    fn check_state(state: AppState<State>) {
        assert_eq!(state.name, "Hello".to_string());
    }

    check_state();
}

#[test]
fn test_non_existent_state() {
    let res = AppState::<MutState>::try_get();
    assert!(res.is_err());
}

#[test]
#[should_panic]
fn test_injected_non_existent_state() {
    #[stateful]
    fn check_state(_state: AppState<MutState>) {}

    check_state();
}

#[test]
fn test_get_mut_state() {
    let mut_state = MutAppState::<MutState>::get();
    assert_eq!(mut_state.get_mut().name, "Hello".to_string());
}

#[test]
fn test_inject_mut_state() {
    #[stateful]
    fn check_mut_state(mut_state: MutAppState<MutState>) {
        assert_eq!(mut_state.get_mut().name, "Hello".to_string());
    }

    check_mut_state();
}

#[test]
fn test_non_existent_mut_state() {
    let res = MutAppState::<State>::try_get();
    assert!(res.is_err());
}

#[test]
#[should_panic]
fn test_injected_non_existent_mut_state() {
    #[stateful]
    fn check_mut_state(_mut_state: MutAppState<State>) {}

    check_mut_state();
}

#[test]
fn test_get_both_state() {
    let state = AppState::<BothState>::get();
    assert_eq!(state.name, "Hello".to_string());
    let mut_state = MutAppState::<BothState>::get();
    assert_eq!(mut_state.get_mut().name, "Hello".to_string());
}

#[test]
fn test_inject_both_state() {
    #[stateful]
    fn check_state(state: AppState<BothState>) {
        assert_eq!(state.name, "Hello".to_string());
    }

    #[stateful]
    fn check_mut_state(mut_state: MutAppState<BothState>) {
        assert_eq!(mut_state.get_mut().name, "Hello".to_string());
    }

    check_state();
    check_mut_state();
}
