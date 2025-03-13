// odpi_rs - a thin wrapper over Oracle Database Programming Interface for C
//
// URL: https://github.com/kubo/odpi_rs
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
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

pub fn must_be_sync(input: TokenStream) -> TokenStream {
    quote! {
        #[maybe_async::must_be_sync]
        #input
    }
}

pub fn maybe_async(_args: TokenStream, input: TokenStream) -> TokenStream {
    must_be_sync(input)
}

pub fn async_impl(_args: TokenStream, _input: TokenStream) -> TokenStream {
    quote! {}
}

pub fn sync_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    must_be_sync(input)
}

pub fn main(_args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    Ok(must_be_sync(input))
}

pub fn test(_args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    Ok(quote! {
        #[maybe_async::must_be_sync]
        #[test]
        #input
    })
}
