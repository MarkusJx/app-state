use app_state::{
    init_default_mut_state, init_default_state, stateful, AppState, AppStateTrait, InitAppState,
    InitMutAppState, MutAppState, MutAppStateLock,
};
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;

#[ctor::ctor]
fn init_logger() {
    // Init logging
    let stdout = ConsoleAppender::builder().build();
    log4rs::init_config(
        Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
            .unwrap(),
    )
    .unwrap();
}

#[derive(InitMutAppState)]
struct MutState {
    name: String,
}

#[derive(InitAppState)]
struct State {
    name: String,
}

#[init_default_state]
#[derive(Default, Debug)]
struct State2 {
    name: String,
}

#[init_default_mut_state]
#[derive(Default, Debug)]
struct MutState2 {
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

#[stateful]
fn check_mut_state(state: MutAppState<MutState>) {
    assert_eq!(state.get_mut().name, "Changed".to_string());
}

#[stateful]
fn with_lock(mut state: MutAppStateLock<MutState>) {
    state.name = "Changed1".to_string();
    assert_eq!(state.name, "Changed1".to_string());
}

#[stateful]
fn check_state(state: AppState<State>) {
    assert_eq!(state.name, "Hello".to_string());
}

#[stateful(init(state))]
fn check_creatable_state(state: AppState<CreatableState>) {
    assert_eq!(state.name, "Hello".to_string());
}

fn main() {
    let state2 = AppState::<State2>::get();
    assert_eq!(state2.get_ref().name, "".to_string());
    let mut_state2 = MutAppState::<MutState2>::get();
    assert_eq!(mut_state2.get_mut().name, "".to_string());

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
