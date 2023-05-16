use std::fmt::Display;

use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;

#[derive(Debug, Clone)]
pub struct SingleMethodParam {
    ty: Ident,
    name: Ident,
}

#[derive(Debug, Clone)]
pub struct MethodParams {
    params: Vec<SingleMethodParam>,
}

impl Display for SingleMethodParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}

impl ToTokens for SingleMethodParam {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(
            format!("{}: {}, ", self.name, self.ty)
                .parse::<TokenStream>()
                .unwrap(),
        );
    }
}

impl Display for MethodParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self
            .params
            .iter()
            .map(|param| param.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", params)
    }
}

impl ToTokens for MethodParams {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for param in &self.params {
            param.to_tokens(tokens);
        }
    }
}

impl Default for MethodParams {
    fn default() -> Self {
        Self { params: vec![] }
    }
}

impl MethodParams {
    pub fn add_param(&mut self, param: SingleMethodParam) {
        self.params.push(param);
    }
}

#[derive(Debug, Clone)]
pub struct ParamBuilder {
    ty: Option<Ident>,
    name: Option<Ident>,
}

impl ParamBuilder {
    pub fn new() -> Self {
        Self {
            ty: None,
            name: None,
        }
    }

    pub fn build(self) -> SingleMethodParam {
        SingleMethodParam {
            ty: self.ty.expect("ty is required"),
            name: self.name.expect("name is required"),
        }
    }

    pub fn ty(mut self, ty: Ident) -> Self {
        self.ty = Some(ty);
        self
    }

    pub fn name(mut self, name: Ident) -> Self {
        self.name = Some(name);
        self
    }

    pub fn can_build(&self) -> bool {
        self.ty.is_some() && self.name.is_some()
    }
}
