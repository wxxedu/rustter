use parser::Parser;
use proc_macro2::TokenStream;
use quote::ToTokens;

extern crate proc_macro;

mod flutter;
mod parser;

#[proc_macro_attribute]
pub fn flutter(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut parser = Parser::new(item.into());
    let flutter = parser.parse();
    let mut token_stream = TokenStream::new();
    flutter.to_tokens(&mut token_stream);
    token_stream.into()
}
