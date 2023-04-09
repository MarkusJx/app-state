use crate::{AppState, AppStateTrait, InitAppState, InitMutAppState, MutAppState};

#[derive(InitAppState, InitMutAppState)]
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

#[test]
fn test_init_state() {
    State::default().init_app_state();
    assert_eq!(AppState::<State>::get().name, "Hello".to_string());
}

#[test]
fn test_init_mut_state() {
    State::default().init_mut_app_state();
    let state = MutAppState::<State>::get();
    let mut state = state.get_mut();

    assert_eq!(state.name, "Hello".to_string());
    state.name = "Changed".to_string();
    drop(state);

    assert_eq!(
        MutAppState::<State>::get().get_mut().name,
        "Changed".to_string()
    );
}
