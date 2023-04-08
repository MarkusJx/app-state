pub(crate) fn is_mut(pat: &Box<syn::Pat>) -> bool {
    if let syn::Pat::Ident(ident) = &**pat {
        ident.mutability.is_some()
    } else {
        false
    }
}
