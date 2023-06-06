use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, Token};

#[derive(Default, Debug)]
pub(crate) struct PathAttr {
    pub(crate) init: Option<Vec<Ident>>,
}

impl Parse for PathAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        const EXPECTED_ATTRIBUTE_MESSAGE: &str = "unexpected identifier, expected any of: init";
        let mut path_attr = PathAttr::default();

        while !input.is_empty() {
            let ident = input.parse::<Ident>().map_err(|error| {
                syn::Error::new(
                    error.span(),
                    format!("{EXPECTED_ATTRIBUTE_MESSAGE}, {error}"),
                )
            })?;
            let attribute_name = &*ident.to_string();

            match attribute_name {
                "init" => {
                    let default;
                    parenthesized!(default in input);

                    path_attr.init = Some(
                        Punctuated::<Ident, Token![,]>::parse_terminated(&default)
                            .map(|punctuated| punctuated.into_iter().collect::<Vec<Ident>>())?,
                    );

                    if path_attr.init.as_ref().unwrap().is_empty() {
                        return Err(syn::Error::new(
                            ident.span(),
                            "expected at least one state to initialize",
                        ));
                    }
                }
                _ => {
                    return Err(syn::Error::new(ident.span(), EXPECTED_ATTRIBUTE_MESSAGE));
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(path_attr)
    }
}
