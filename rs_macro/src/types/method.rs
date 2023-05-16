use super::{param::MethodParams, return_type::ReturnType};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Method {
    name: Ident,
    args: MethodParams,
    return_type: ReturnType,
    body: TokenStream,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "pub fn {}({}){} {{ {} }}",
            self.name, self.args, self.return_type, self.body
        )
    }
}

impl ToTokens for Method {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let args = &self.args;
        let return_type = &self.return_type;
        let body = &self.body;
        tokens.extend(quote! {
            #[no_mangle]
            pub extern "C" fn #name(#args) #return_type {
                #body
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct MethodBuilder {
    name: Option<Ident>,
    args: Option<MethodParams>,
    return_type: Option<ReturnType>,
    body: Option<TokenStream>,
}

impl MethodBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            args: None,
            return_type: None,
            body: None,
        }
    }

    pub fn build(self) -> Method {
        Method {
            name: self.name.expect("name is required"),
            args: self.args.expect("args is required"),
            return_type: self.return_type.expect("return_type is required"),
            body: self.body.expect("body is required"),
        }
    }

    pub fn name(mut self, name: Ident) -> Self {
        self.name = Some(name);
        self
    }

    pub fn args(mut self, args: MethodParams) -> Self {
        self.args = Some(args);
        self
    }

    pub fn return_type(mut self, return_type: ReturnType) -> Self {
        self.return_type = Some(return_type);
        self
    }

    pub fn body(mut self, body: TokenStream) -> Self {
        self.body = Some(body);
        self
    }
}
