use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use quote::ToTokens;

use crate::types::{
    method::{Method, MethodBuilder},
    param::{MethodParams, ParamBuilder},
    return_type::ReturnType,
};

#[derive(Debug)]
pub(crate) struct Parser {
    tokens: Vec<TokenTree>,
}

impl Parser {
    pub(crate) fn new(iter: TokenStream) -> Self {
        let mut iter = iter.into_iter().collect::<Vec<_>>();
        iter.reverse();
        Self { tokens: iter }
    }

    pub(crate) fn parse(&mut self) -> Method {
        let mut builder = MethodBuilder::new();
        let name = self.parse_name().expect("name is required");
        builder = builder.name(name);
        let args = self.parse_args();
        builder = builder.args(args);
        let return_type = self.parse_return_type();
        builder = builder.return_type(return_type);
        let body = self.parse_body();
        builder = builder.body(body);
        let res = builder.build();
        res
    }

    /// Returns the name of the function, which is the first `Ident` after the
    /// `fn` keyword.
    fn parse_name(&mut self) -> Option<Ident> {
        while let Some(token) = self.tokens.pop() {
            match token {
                TokenTree::Ident(ident) => {
                    if ident.to_string() == "fn" {
                        break;
                    }
                }
                _ => {}
            }
        }
        if let Some(TokenTree::Ident(ident)) = self.tokens.pop() {
            Some(ident)
        } else {
            None
        }
    }

    /// Returns the arguments of the function.
    fn parse_args(&mut self) -> MethodParams {
        // read until the first group
        let mut params = MethodParams::default();
        while let Some(token) = self.tokens.pop() {
            match token {
                TokenTree::Group(ident) => {
                    self.parse_arg_groups(&mut params, &mut ident.clone());
                    break;
                }
                _ => {
                    panic!("unexpected token");
                }
            }
        }
        params
    }

    fn parse_arg_groups(
        &mut self,
        params: &mut MethodParams,
        group: &mut Group,
    ) {
        let mut param_iter = group.stream().into_iter().peekable();
        let mut param_builder = ParamBuilder::new();
        while let Some(token) = param_iter.next() {
            match token {
                TokenTree::Ident(ident) => {
                    param_builder = param_builder.name(ident);
                }
                TokenTree::Punct(punct) => {
                    if punct.to_string() == ":" {
                        match param_iter.next() {
                            Some(TokenTree::Ident(ident)) => {
                                param_builder = param_builder.ty(ident);
                            }
                            _ => {
                                panic!("expected type after :");
                            }
                        }
                    } else if punct.to_string() == "," {
                        params.add_param(param_builder.build());
                        param_builder = ParamBuilder::new();
                    }
                }
                _ => {
                    panic!("unexpected token");
                }
            }
        }
        if param_builder.can_build() {
            params.add_param(param_builder.build());
        }
    }

    fn parse_return_type(&mut self) -> ReturnType {
        // let mut tokens = Vec::new();
        // let mut is_return_type = false;
        // while let Some(token) = self.tokens.pop() {
        //     match token {
        //         TokenTree::Punct(punct) => {
        //             if punct.to_string() == "-" {
        //                 match self.tokens.pop() {
        //                     Some(TokenTree::Punct(punct)) => {
        //                         if punct.to_string() == ">" {
        //                             is_return_type = true;
        //                             break;
        //                         }
        //                     }
        //                     token => {
        //                         tokens.push(TokenTree::Punct(punct));
        //                         tokens.push(token.unwrap());
        //                     }
        //                 }
        //             } else {
        //                 tokens.push(TokenTree::Punct(punct));
        //             }
        //         }
        //         token => {
        //             tokens.push(token);
        //             break;
        //         }
        //     }
        // }
        // if !is_return_type {
        //     // push back the tokens
        //     for token in tokens.into_iter().rev() {
        //         self.tokens.push(token);
        //     }
        //     return ReturnType::Void;
        // }
        let mut stream = TokenStream::new();
        while self.tokens.len() > 1 {
            let token = self.tokens.pop().unwrap();
            stream.extend(TokenStream::from(token));
        }
        ReturnType::ReturnType(stream)
    }

    fn parse_body(&mut self) -> TokenStream {
        let mut stream = TokenStream::new();
        while let Some(token) = self.tokens.pop() {
            match token {
                TokenTree::Group(group) => {
                    for token in group.stream().into_iter() {
                        stream.extend(TokenStream::from(token));
                    }
                }
                _ => {}
            }
        }
        stream
    }
}
