extern crate proc_macro;
use quote::ToTokens;

use proc_macro::TokenStream as RawStream;
use proc_macro2::TokenStream;
use quote::quote;
use std::error::Error;
use syn::PathSegment;

/// Derive macro for `InitAppState`.
/// Allows you to initialize app states with `init_app_state`.
///
/// # Example
/// ```
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
/// ```
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
/// ```
/// use app_state::{AppState, MutAppState, stateful};
///
/// struct SomeState;
/// struct SomeMutState;
/// struct SomeOtherState;
///
/// #[stateful]
/// fn foo(app_state: AppState<SomeState>,
///        mut_app_state: MutAppState<SomeMutState>,
///        mut other_state: MutAppStateLock<SomeOtherState>) {
///     // ...
/// }
///
/// fn main() {
///    AppState::init(SomeState);
///    MutAppState::init(SomeMutState);
///    MutAppState::init(SomeOtherState);
///
///    foo();
/// }
/// ```
#[proc_macro_attribute]
pub fn stateful(_args: RawStream, input: RawStream) -> RawStream {
    expand(input.into()).unwrap().to_token_stream().into()
}

#[derive(Eq, PartialEq)]
enum StateIdent {
    AppState,
    MutAppState,
    MutAppStateLock { is_mut: bool },
}

impl StateIdent {
    pub fn new(segment: &PathSegment, pat: &Box<syn::Pat>) -> Self {
        match segment.ident.to_string().as_str() {
            "AppState" => StateIdent::AppState,
            "MutAppState" => StateIdent::MutAppState,
            "MutAppStateLock" => {
                let is_mut = if let syn::Pat::Ident(ident) = &**pat {
                    ident.mutability.is_some()
                } else {
                    false
                };

                StateIdent::MutAppStateLock { is_mut }
            }
            _ => panic!("Invalid state type"),
        }
    }

    pub fn to_token_stream(&self) -> TokenStream {
        match self {
            StateIdent::AppState => quote! { AppState },
            StateIdent::MutAppState => quote! { MutAppState },
            StateIdent::MutAppStateLock { .. } => quote! { MutAppStateLock },
        }
    }
}

fn get_type(
    input: &syn::FnArg,
) -> Result<Option<(TokenStream, StateIdent, TokenStream)>, Box<dyn Error>> {
    if let syn::FnArg::Typed(typed) = input {
        let name = if let syn::Pat::Ident(ident) = &*typed.pat {
            ident.ident.to_string().parse::<TokenStream>()?
        } else {
            return Ok(None);
        };

        if let syn::Type::Path(path) = &*typed.ty {
            for segment in &path.path.segments {
                if segment.ident == "AppState"
                    || segment.ident == "MutAppState"
                    || segment.ident == "MutAppStateLock"
                {
                    let state_type = StateIdent::new(segment, &typed.pat);
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        for arg in &args.args {
                            if let syn::GenericArgument::Type(syn::Type::Path(path)) = arg {
                                let ident = path.path.segments[0].ident.to_string().parse()?;
                                return Ok(Some((name, state_type, ident)));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}

fn expand(input: TokenStream) -> Result<TokenStream, Box<dyn Error>> {
    let mut item = syn::parse2::<syn::Item>(input)?;

    if let syn::Item::Fn(ref mut item) = item {
        let states = item
            .sig
            .inputs
            .iter()
            .map(|input| get_type(&input))
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?
            .into_iter()
            .filter_map(|x| x)
            .collect::<Vec<_>>();

        item.sig.inputs = item
            .sig
            .inputs
            .clone()
            .into_iter()
            .map(|input| get_type(&input).map(|x| (input, x)))
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?
            .into_iter()
            .filter(|x| x.1.is_none())
            .map(|x| x.0)
            .collect::<_>();

        let mut statements = Vec::new();
        for (var_name, state_type, type_name) in states {
            if let StateIdent::MutAppStateLock { is_mut } = state_type {
                statements.push(syn::parse2::<syn::Stmt>(quote! {
                    let #var_name = MutAppState::<#type_name>::get();
                })?);

                if is_mut {
                    statements.push(syn::parse2::<syn::Stmt>(quote! {
                        let mut #var_name = MutAppStateLock::new(&#var_name);
                    })?);
                } else {
                    statements.push(syn::parse2::<syn::Stmt>(quote! {
                        let #var_name = MutAppStateLock::new(&#var_name);
                    })?);
                }
            } else {
                let state_type = state_type.to_token_stream();
                statements.push(syn::parse2::<syn::Stmt>(quote! {
                    let #var_name = #state_type::<#type_name>::get();
                })?);
            }
        }

        statements.append(&mut item.block.stmts);
        item.block.stmts = statements;
    } else {
        panic!("stateful can only be used on functions");
    }

    Ok(item.to_token_stream())
}
