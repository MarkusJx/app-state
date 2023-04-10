mod util;

extern crate proc_macro;

use crate::util::stateful::expand_stateful;
use proc_macro::TokenStream as RawStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use rand::Rng;
use syn::DeriveInput;

/// Derive macro for `InitAppState`.
/// Allows you to initialize app states with `init_app_state`.
///
/// # Example
/// ```no_run
/// use app_state::{stateful, AppState, InitAppState};
///
/// #[derive(InitAppState)]
/// struct State {
///   name: String,
/// }
///
/// fn main() {
///   State {
///     name: "Hello".to_string(),
///   }.init_app_state();
/// }
/// ```
#[proc_macro_derive(InitAppState)]
pub fn init_app_state(input: RawStream) -> RawStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;
    let gen = quote! {
        impl InitAppState for #name {
            fn init_app_state(self) {
                AppState::init(self);
            }
        }
    };
    gen.into()
}

/// Derive macro for `InitMutAppState`.
/// Allows you to initialize app states with `init_mut_app_state`.
///
/// # Example
/// ```no_run
/// use app_state::{stateful, MutAppState, InitMutAppState};
///
/// #[derive(InitMutAppState)]
/// struct State {
///   name: String,
/// }
///
/// fn main() {
///   State {
///     name: "Hello".to_string(),
///   }.init_mut_app_state();
/// }
/// ```
#[proc_macro_derive(InitMutAppState)]
pub fn init_mut_app_state(input: RawStream) -> RawStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;
    let gen = quote! {
        impl InitMutAppState for #name {
            fn init_mut_app_state(self) {
                MutAppState::init(self);
            }
        }
    };
    gen.into()
}

/// Inject app states into the annotated function.
///
/// # Example
/// ```no_run
/// use app_state::{AppState, MutAppState, stateful};
///
/// struct SomeState;
/// struct SomeMutState;
/// struct SomeOtherState;
///
/// #[stateful]
/// fn foo(app_state: AppState<SomeState>,
///   mut_app_state: MutAppState<SomeMutState>,
///   mut other_state: MutAppStateLock<SomeOtherState>) {
///   // ...
/// }
///
/// fn main() {
///   AppState::init(SomeState);
///   MutAppState::init(SomeMutState);
///   MutAppState::init(SomeOtherState);
///
///   foo();
/// }
/// ```
#[proc_macro_attribute]
pub fn stateful(_args: RawStream, input: RawStream) -> RawStream {
    expand_stateful(input.into())
        .unwrap()
        .to_token_stream()
        .into()
}

fn get_default_state_values(input: DeriveInput) -> (Ident, Ident) {
    let name = input.ident.clone();

    let mut rng = rand::thread_rng();
    let id = Ident::new(
        &format!("__init_default_state_{}", rng.gen::<u32>()),
        proc_macro2::Span::call_site(),
    );

    if let syn::Item::Struct(_) = input.clone().into() {
        (name, id)
    } else {
        panic!("`init_default_state` can only be used on structs");
    }
}

/// Initialize the default state of the annotated struct
/// on application startup using `ctor`.
/// The default state is the result of calling `Default::default()`.
///
/// # Example
/// ```no_run
/// use app_state::{MutAppState, init_default_state, AppStateTrait};
///
/// #[init_default_state]
/// #[derive(Default)]
/// struct SomeState {
///   name: String,
/// }
/// ```
#[proc_macro_attribute]
pub fn init_default_state(_: RawStream, input: RawStream) -> RawStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let (name, id) = get_default_state_values(input.clone());

    (quote! {
        #input

        #[ctor::ctor]
        fn #id() {
            AppState::init(#name::default());
        }
    })
    .into()
}

/// Initialize the default state of the annotated struct
/// on application startup using `ctor`.
/// The default state is the result of calling `Default::default()`.
///
/// # Example
/// ```no_run
/// use app_state::{MutAppState, init_default_mut_state, AppStateTrait};
///
/// #[init_default_mut_state]
/// #[derive(Default)]
/// struct SomeState {
///   name: String,
/// }
/// ```
#[proc_macro_attribute]
pub fn init_default_mut_state(_: RawStream, input: RawStream) -> RawStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let (name, id) = get_default_state_values(input.clone());

    (quote! {
        #input

        #[ctor::ctor]
        fn #id() {
            MutAppState::init(#name::default());
        }
    })
    .into()
}
