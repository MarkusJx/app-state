//! Thread-safe, mutable application states for rust.
//!
//! # Examples
//! ## Initializing the state
//! Any app state that may be used must be initialized first.
//! **Note:** An `AppState` can not be used as a `MutAppState` and vice versa.
//! This means that `AppState` and `MutAppState` must be initialized separately
//! and use independent values, even if they are of the same type.
//!
//! ```rust
//! use app_state::{AppState, MutAppState, AppStateTrait};
//!
//! struct MyState {
//!  counter: u32,
//! }
//!
//! fn main() {
//!   // Initialize the app state
//!   AppState::init(MyState { counter: 0 });
//!   // Initialize the mutable app state
//!   MutAppState::init(MyState { counter: 0 });
//! }
//! ```
//!
//! ### Using `derive`
//! In order to avoid boilerplate code, the `InitAppState` and `InitMutAppState`
//! traits can be derived for any struct. These traits provide the `init_app_state`
//! and `init_mut_app_state` methods respectively which can be used to initialize
//! the state more easily.
//! ```rust
//! use app_state::{AppState, MutAppState, AppStateTrait, InitAppState, InitMutAppState};
//!
//! #[derive(Default, InitAppState, InitMutAppState)]
//! struct MyState {
//!   counter: u32,
//! }
//!
//! fn main() {
//!   MyState::default().init_app_state();
//!   MyState::default().init_mut_app_state();
//! }
//! ```
//!
//! ## Read-only state
//! App states internally use `Arc` to allow for thread-safe access.
//! ```rust
//! use app_state::{AppState, AppStateTrait, stateful};
//!
//! struct MyState {
//!   counter: u32,
//! }
//!
//! #[stateful]
//! fn func(state: AppState<MyState>) {
//!   println!("Counter: {}", state.counter);
//! }
//! ```
//!
//! ## Mutable state
//! Mutable states internally use a `Mutex` to ensure thread-safety.
//! This means that when reading from or writing to the state, the mutex must be locked.
//! This can be done either by calling `get_mut()` or by using the `MutAppStateLock` type.
//! ```rust
//! use app_state::{MutAppState, AppStateTrait, stateful};
//!
//! struct MyState {
//!   counter: u32,
//! }
//!
//! #[stateful]
//! fn func(state: MutAppState<MyState>) {
//!   let mut state = state.get_mut();
//!   state.counter += 1;
//! }
//! ```
//!
//! ## Mutable state (locked)
//! In order to mutate the state, you must first lock it.
//! This can be done either by calling `get_mut()` or by using the `MutAppStateLock` type.
//! ```rust
//! use app_state::{MutAppState, MutAppStateLock, AppStateTrait, stateful};
//!
//! struct MyState {
//!   counter: u32,
//! }
//!
//! #[stateful]
//! fn func(mut state: MutAppStateLock<MyState>) {
//!   state.counter += 1;
//! }
//! ```
//!
//! ## Get the state manually
//! You can also get the state manually by calling `AppState::get()` or `MutAppState::get()`.
//! ```no_run
//! use app_state::{AppState, MutAppState, AppStateTrait};
//!
//! struct MyState {
//!   counter: u32,
//! }
//!
//! fn main() {
//!   let state = AppState::<MyState>::get();
//!   let mut_state = MutAppState::<MyState>::get();
//! }
//! ```

mod states;

#[cfg(test)]
mod tests;

pub use crate::states::app_state::*;
pub use crate::states::mut_app_state_lock::*;
pub use crate::states::mutable_app_state::*;
pub use crate::states::traits::*;
pub use app_state_macros::*;
