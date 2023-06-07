use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, Token};

#[derive(Default, Debug)]
pub(crate) struct PathAttr {
    pub(crate) init: Option<Vec<Ident>>,
    #[cfg(feature = "log")]
    pub(crate) log_member: Option<Ident>,
    #[cfg(feature = "log")]
    pub(crate) no_log: Option<Ident>,
}

impl Parse for PathAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        const EXPECTED_ATTRIBUTE_MESSAGE: &str =
            "unexpected identifier, expected any of: init, log_member, no_log";
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
                #[cfg(feature = "log")]
                "log_member" => {
                    path_attr.log_member = Some(Ident::new("log_member", ident.span()));
                }
                #[cfg(feature = "log")]
                "no_log" => {
                    path_attr.no_log = Some(Ident::new("no_log", ident.span()));
                }
                _ => {
                    return Err(syn::Error::new(ident.span(), EXPECTED_ATTRIBUTE_MESSAGE));
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        #[cfg(feature = "log")]
        if path_attr.no_log.is_some() && path_attr.log_member.is_some() {
            return Err(syn::Error::new(
                path_attr.no_log.unwrap().span(),
                "cannot use both no_log and log_member",
            ));
        }

        Ok(path_attr)
    }
}
