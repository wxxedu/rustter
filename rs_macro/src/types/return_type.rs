use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ReturnType {
    Void,
    ReturnType(TokenStream),
}

impl Display for ReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReturnType::Void => write!(f, "void"),
            ReturnType::ReturnType(t) => write!(f, "{}", t),
        }
    }
}

impl ToTokens for ReturnType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ReturnType::Void => tokens.extend(quote! {}),
            ReturnType::ReturnType(t) => tokens.extend(quote! { #t }),
        }
    }
}
