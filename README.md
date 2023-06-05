# app-state

Thread-safe, mutable application states for rust.

## Usage
```rust
use app_state::{stateful, AppState, InitAppState, InitMutAppState, MutAppState, MutAppStateLock};

#[derive(InitMutAppState)]
struct MutState {
    name: String,
}

#[derive(InitAppState)]
struct State {
    name: String,
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

/// Injects the state into the function and
/// returns a mutable reference to the state.
/// The state must be locked manually before it can be used.
#[stateful]
fn check_mut_state(state: MutAppState<MutState>) {
    assert_eq!(state.get_mut().name, "Changed".to_string());
}

/// Locks the state and returns a mutable reference to the state.
/// May cause a deadlock if the state is already locked.
#[stateful]
fn with_lock(mut state: MutAppStateLock<MutState>) {
    state.name = "Changed1".to_string();
    assert_eq!(state.name, "Changed1".to_string());
}

/// Injects the state into the function.
#[stateful]
fn check_state(state: AppState<State>) {
    assert_eq!(state.name, "Hello".to_string());
}

/// Initializes the state with the default value if
/// the state is not already initialized.
#[stateful(default(state))]
fn check_creatable_state(state: AppState<CreatableState>) {
    assert_eq!(state.name, "Hello".to_string());
}

fn main() {
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
```