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
use quote::{quote, ToTokens};
use syn::{ItemFn, Result};

pub fn must_be_async(args: TokenStream, input: TokenStream) -> TokenStream {
    quote! {
        #[maybe_async::must_be_async(#args)]
        #input
    }
}

pub fn maybe_async(args: TokenStream, input: TokenStream) -> TokenStream {
    must_be_async(args, input)
}

pub fn async_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    must_be_async(args, input)
}

pub fn sync_impl(_args: TokenStream, _input: TokenStream) -> TokenStream {
    quote! {}
}

fn block_on(input: TokenStream, pre_tokens: TokenStream) -> Result<TokenStream> {
    let mut item: ItemFn = syn::parse2(input.clone())?;
    item.sig.asyncness = None;
    let attrs = &item.attrs;
    let vis = &item.vis;
    let sig = &item.sig;
    let body = task::block_on(item.block.to_token_stream());

    Ok(quote! {
        #pre_tokens
        #[maybe_async::must_be_async]
        #(#attrs)*
        #vis #sig {
            #body
        }
    })
}

pub fn main(_args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    block_on(input, quote!())
}

pub fn test(_args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    block_on(input, quote!(#[test]))
}

#[cfg(feature = "async-std")]
mod task {
    use proc_macro2::TokenStream;
    use quote::quote;

    pub fn block_on(body: TokenStream) -> TokenStream {
        quote! {
            async_std::task::block_on(async #body)
        }
    }
}

#[cfg(feature = "smol")]
mod task {
    use proc_macro2::TokenStream;
    use quote::quote;

    pub fn block_on(body: TokenStream) -> TokenStream {
        quote! {
            smol::block_on(async #body)
        }
    }
}

#[cfg(feature = "tokio")]
mod task {
    use proc_macro2::TokenStream;
    use quote::quote;

    pub fn block_on(body: TokenStream) -> TokenStream {
        quote! {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async #body)
        }
    }
}
