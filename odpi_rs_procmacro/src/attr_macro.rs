// Rust-oracle - Rust binding for Oracle database
//
// URL: https://github.com/kubo/rust-oracle
//
//-----------------------------------------------------------------------------
// Copyright (c) 2025 Kubo Takehiro <kubo@jiubao.org>. All rights reserved.
// This program is free software: you can modify it and/or redistribute it
// under the terms of:
//
// (i)  the Universal Permissive License v 1.0 or at your option, any
//      later version (http://oss.oracle.com/licenses/upl); and/or
//
// (ii) the Apache License v 2.0. (http://www.apache.org/licenses/LICENSE-2.0)
//-----------------------------------------------------------------------------
use crate::{AsyncRuntime, RT};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Error, Ident, ItemFn, Result, ReturnType};

// macro argument
#[derive(Debug, PartialEq)]
enum Args {
    // #[oracle::main(async)] or #[oracle::test(async)]
    Async,

    // #[oracle::main(maybe_async)] or #[oracle::test(maybe_async)]
    MaybeAsync,

    // #[oracle::main(sync)] or #[oracle::test(sync)]
    Sync,
}

// #[oracle::main] or #[oracle::test]
enum Method {
    Main,
    Test,
}

impl Args {
    fn from_token_streams(args: TokenStream) -> Result<Args> {
        let args_span = args.span();
        let arg = syn::parse2::<Ident>(args)?.to_string();
        match arg.as_str() {
            "maybe_async" => Ok(Args::MaybeAsync),
            "async" => Ok(Args::Async),
            "sync" => Ok(Args::Sync),
            _ => Err(Error::new(
                args_span,
                "only accepts `maybe_async`, `async`, `sync`",
            )),
        }
    }

    fn make_body_blank(&self) -> bool {
        match self {
            Args::Async => RT.is_sync(),
            Args::MaybeAsync => false,
            Args::Sync => RT.is_async(),
        }
    }

    fn should_convert_async_to_sync(&self) -> bool {
        match self {
            Args::Async => false,
            Args::MaybeAsync => RT.is_sync(),
            Args::Sync => false,
        }
    }
}

fn apply_attributes(args: TokenStream, input: TokenStream, method: Method) -> Result<TokenStream> {
    let args = Args::from_token_streams(args)?;
    let opt_must_be_sync_attr = if args.should_convert_async_to_sync() {
        Some(quote! {#[maybe_async::must_be_sync]})
    } else {
        None
    };
    let opt_test_attr = match method {
        Method::Main => None,
        Method::Test => Some(quote! {#[test]}),
    };
    let item: ItemFn = syn::parse2(input.clone())?;
    let attrs = &item.attrs;
    let vis = &item.vis;
    let name = &item.sig.ident;
    let ret = &item.sig.output;
    let body = &item.block;

    if !item.sig.inputs.is_empty() {
        let func_type = match method {
            Method::Main => "the main function",
            Method::Test => "test functions",
        };
        return Err(Error::new_spanned(
            item.sig.inputs,
            format!("{} cannot accept arguments", func_type),
        ));
    }

    let body = if args.make_body_blank() {
        if item.sig.output == ReturnType::Default {
            quote!(())
        } else {
            quote!(Ok(()))
        }
    } else {
        match RT {
            AsyncRuntime::Tokio => {
                quote! {
                    tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .unwrap()
                        .block_on(async #body)
                }
            }
            AsyncRuntime::AsyncStd => {
                quote! {
                    async_std::task::block_on(async #body)
                }
            }
            AsyncRuntime::Smol => {
                quote! {
                    smol::block_on(async #body)
                }
            }
            AsyncRuntime::None => {
                quote! {
                    #body
                }
            }
        }
    };
    Ok(quote! {
        #opt_must_be_sync_attr
        #opt_test_attr
        #(#attrs)*
        #vis fn #name() #ret {
            #body
        }
    })
}

pub fn maybe_async(args: TokenStream, input: TokenStream) -> TokenStream {
    if RT.is_async() {
        quote! {
            #[maybe_async::must_be_async(#args)]
            #input
        }
    } else {
        quote! {
            #[maybe_async::must_be_sync(#args)]
            #input
        }
    }
}

pub fn main(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    apply_attributes(args, input, Method::Main)
}

pub fn test(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    apply_attributes(args, input, Method::Test)
}
