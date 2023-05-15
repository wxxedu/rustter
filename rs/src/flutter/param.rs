use std::fmt::Display;

use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;

#[derive(Debug, Clone)]
pub struct FlutterParam {
    ty: Ident,
    name: Ident,
}

#[derive(Debug, Clone)]
pub struct FlutterParams {
    params: Vec<FlutterParam>,
}

impl Display for FlutterParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}

impl ToTokens for FlutterParam {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(
            format!("{}: {}, ", self.name, self.ty)
                .parse::<TokenStream>()
                .unwrap(),
        );
    }
}

impl Display for FlutterParams {
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

impl ToTokens for FlutterParams {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for param in &self.params {
            param.to_tokens(tokens);
        }
    }
}

impl Default for FlutterParams {
    fn default() -> Self {
        Self { params: vec![] }
    }
}

impl FlutterParams {
    pub fn add_param(&mut self, param: FlutterParam) {
        self.params.push(param);
    }
}

#[derive(Debug, Clone)]
pub struct FlutterParamBuilder {
    ty: Option<Ident>,
    name: Option<Ident>,
}

impl FlutterParamBuilder {
    pub fn new() -> Self {
        Self {
            ty: None,
            name: None,
        }
    }

    pub fn build(self) -> FlutterParam {
        FlutterParam {
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
}
