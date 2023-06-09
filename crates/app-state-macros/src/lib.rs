mod util;

extern crate proc_macro;

use crate::util::path::PathAttr;
use crate::util::stateful::expand_stateful;
use proc_macro::TokenStream as RawStream;
use proc_macro2::Ident;
use quote::quote;
use rand::Rng;
use std::env;
use syn::spanned::Spanned;
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

    #[cfg(feature = "log")]
    let log = quote! {
        log::debug!("Initializing app state {}", std::any::type_name::<#name>());
    };
    #[cfg(not(feature = "log"))]
    let log = quote! {};

    let gen = quote! {
        impl InitAppState for #name {
            fn init_app_state(self) {
                #log
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

    #[cfg(feature = "log")]
    let log = quote! {
        log::debug!("Initializing mutable app state {}", std::any::type_name::<#name>());
    };
    #[cfg(not(feature = "log"))]
    let log = quote! {};

    let gen = quote! {
        impl InitMutAppState for #name {
            fn init_mut_app_state(self) {
                #log
                MutAppState::init(self);
            }
        }
    };
    gen.into()
}

/// Inject app states into the annotated function.
///
/// # Arguments
/// ## `init`
/// A list of argument names to initialize to their default values
/// if they are not already initialized. This requires
/// the specified states to implement `Default`.
///
/// # Examples
/// ## Injecting multiple states
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
///
/// ## Injecting states with default values
/// ```no_run
/// use app_state::{AppState, MutAppState, stateful};
///
/// #[derive(Default)]
/// struct SomeState;
/// #[derive(Default)]
/// struct SomeMutState;
/// #[derive(Default)]
/// struct SomeOtherState;
///
/// #[stateful(init(app_state, mut_app_state, other_state))]
/// fn foo(app_state: AppState<SomeState>,
///   mut_app_state: MutAppState<SomeMutState>,
///   mut other_state: MutAppStateLock<SomeOtherState>) {
///   // ...
/// }
///
/// fn main() {
///   // All states will be initialized with their default values
///   // if they are not already initialized.
///   foo();
/// }
/// ```
#[proc_macro_attribute]
pub fn stateful(args: RawStream, input: RawStream) -> RawStream {
    let args = syn::parse_macro_input!(args as PathAttr);

    match expand_stateful(input.into(), args) {
        Ok(stream) => {
            if env::var("DEBUG_GENERATED_CODE").is_ok() {
                println!("{}", stream.to_string());
            }

            stream.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}

fn get_default_state_values(input: DeriveInput) -> syn::Result<(Ident, Ident)> {
    let name = input.ident.clone();

    let mut rng = rand::thread_rng();
    let id = Ident::new(
        &format!("__init_default_state_{}", rng.gen::<u32>()),
        proc_macro2::Span::call_site(),
    );

    if let syn::Item::Struct(_) = input.clone().into() {
        Ok((name, id))
    } else {
        Err(syn::Error::new(
            input.span(),
            "'init_default_state' can only be used on structs",
        ))
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
    let (name, id) = match get_default_state_values(input.clone()) {
        Ok(res) => res,
        Err(err) => return err.to_compile_error().into(),
    };

    #[cfg(feature = "log")]
    let log = quote! {
        log::debug!("Initializing app state {} before main", std::any::type_name::<#name>());
    };
    #[cfg(not(feature = "log"))]
    let log = quote! {};

    (quote! {
        #input

        #[ctor::ctor]
        fn #id() {
            #log
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
    let (name, id) = match get_default_state_values(input.clone()) {
        Ok(res) => res,
        Err(err) => return err.to_compile_error().into(),
    };

    #[cfg(feature = "log")]
    let log = quote! {
        log::debug!("Initializing mutable app state {} before main", std::any::type_name::<#name>());
    };
    #[cfg(not(feature = "log"))]
    let log = quote! {};

    (quote! {
        #input

        #[ctor::ctor]
        fn #id() {
            #log
            MutAppState::init(#name::default());
        }
    })
    .into()
}
