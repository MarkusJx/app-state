use crate::util::path::PathAttr;
use crate::util::util::is_mut;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::PathSegment;

#[derive(Eq, PartialEq)]
enum StateIdent {
    AppState,
    MutAppState,
    MutAppStateLock,
}

impl StateIdent {
    pub fn new(segment: &PathSegment) -> Self {
        match segment.ident.to_string().as_str() {
            "AppState" => StateIdent::AppState,
            "MutAppState" => StateIdent::MutAppState,
            "MutAppStateLock" => StateIdent::MutAppStateLock,
            _ => panic!("Invalid state type"),
        }
    }

    pub fn to_token_stream(&self) -> TokenStream {
        match self {
            StateIdent::AppState => quote! { AppState },
            StateIdent::MutAppState => quote! { MutAppState },
            StateIdent::MutAppStateLock => quote! { MutAppStateLock },
        }
    }
}

fn get_type(
    input: &syn::FnArg,
) -> Result<Option<(TokenStream, StateIdent, TokenStream, TokenStream)>, syn::Error> {
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
                    let state_type = StateIdent::new(segment);
                    let is_mut = if is_mut(&typed.pat) {
                        quote! { mut }
                    } else {
                        quote! {}
                    };

                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        for arg in &args.args {
                            if let syn::GenericArgument::Type(syn::Type::Path(path)) = arg {
                                let ident = path.path.segments[0].ident.to_string().parse()?;
                                return Ok(Some((name, state_type, ident, is_mut)));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}

fn is_default(args: &PathAttr, name: &TokenStream) -> bool {
    args.default
        .as_ref()
        .map(|d| d.iter().any(|x| x.to_string() == name.to_string()))
        .unwrap_or(false)
}

pub(crate) fn expand_stateful(
    input: TokenStream,
    args: PathAttr,
) -> Result<TokenStream, syn::Error> {
    let mut item = syn::parse2::<syn::Item>(input)?;

    if let syn::Item::Fn(ref mut item) = item {
        let states = item
            .sig
            .inputs
            .iter()
            .map(|input| get_type(&input))
            .collect::<Result<Vec<_>, syn::Error>>()?
            .into_iter()
            .filter_map(|x| x)
            .collect::<Vec<_>>();

        item.sig.inputs = item
            .sig
            .inputs
            .clone()
            .into_iter()
            .map(|input| get_type(&input).map(|x| (input, x)))
            .collect::<Result<Vec<_>, syn::Error>>()?
            .into_iter()
            .filter(|x| x.1.is_none())
            .map(|x| x.0)
            .collect::<_>();

        // Check if all arguments marked as default are present
        if let Some(not_found) = args.default.as_ref().and_then(|d| {
            d.iter()
                .find(|e1| !states.iter().any(|e2| e1.to_string() == e2.0.to_string()))
        }) {
            return Err(syn::Error::new(
                not_found.span(),
                format!(
                    "Argument named '{}' not found",
                    not_found.to_token_stream().to_string()
                ),
            )
            .into());
        }

        let mut statements = Vec::new();
        for (var_name, state_type, type_name, is_mut) in states {
            let state_type_tokens = state_type.to_token_stream();

            if state_type == StateIdent::MutAppStateLock {
                if is_default(&args, &var_name) {
                    statements.push(syn::parse2::<syn::Stmt>(quote! {
                        MutAppState::init_if_not_exists(#type_name::default);
                    })?);
                }

                statements.push(syn::parse2::<syn::Stmt>(quote! {
                    let #var_name = MutAppState::<#type_name>::get();
                })?);

                statements.push(syn::parse2::<syn::Stmt>(quote! {
                    let #is_mut #var_name = MutAppStateLock::new(&#var_name);
                })?);
            } else {
                if is_default(&args, &var_name) {
                    statements.push(syn::parse2::<syn::Stmt>(quote! {
                        #state_type_tokens::init_if_not_exists(#type_name::default);
                    })?);
                }

                statements.push(syn::parse2::<syn::Stmt>(quote! {
                    let #is_mut #var_name = #state_type_tokens::<#type_name>::get();
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
