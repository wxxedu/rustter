use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum FlutterReturnType {
    Void,
    ReturnType(TokenStream),
}

impl Display for FlutterReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FlutterReturnType::Void => write!(f, "void"),
            FlutterReturnType::ReturnType(t) => write!(f, "{}", t),
        }
    }
}

impl ToTokens for FlutterReturnType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FlutterReturnType::Void => tokens.extend(quote! {}),
            FlutterReturnType::ReturnType(t) => tokens.extend(quote! { -> #t }),
        }
    }
}
