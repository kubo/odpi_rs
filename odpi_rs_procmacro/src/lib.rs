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

//! Procedural macros for odpi_rs
use odpic_sys::doc::OdpicDoc;
use proc_macro::TokenStream;
use std::sync::LazyLock;
use syn::Error;

mod odpic_doc;
#[cfg_attr(feature = "is_async", path = "runtime/async.rs")]
#[cfg_attr(not(feature = "is_async"), path = "runtime/sync.rs")]
mod runtime;

/// Unifies async and sync implementation.
///
/// When one of async runtime feature flags is set, this is replaced with
/// [`#[maybe_async::must_be_async]`][maybe_async]. The attribute arguments are passed
/// to the macro as they are.
///
/// Otherwise, it is replaced with [`#[maybe_async::must_be_sync]`][maybe_async].
///
/// # Examples
///
/// ```
/// # use odpi_rs_procmacro as odpi_rs;
/// # #[odpi_rs::maybe_async]
/// # async fn async_fn() -> bool { true }
/// #[odpi_rs::maybe_async]
/// async fn my_func() {
///     let res = async_fn().await;
///     assert!(res);
/// }
/// ```
///
/// Equivalent code when one of async feature flags is set
///
/// ```
/// # async fn async_fn() -> bool { true }
/// // features = ["tokio"] or other async runtime
/// async fn my_func() {
///     let res = async_fn().await;
///     assert!(res);
/// }
/// ```
///
/// Equivalent code when none of async feature flags are set
///
/// ```
/// # fn async_fn() -> bool { true }
/// // features = []
/// fn my_func() {
///     let res = async_fn();
///     assert!(res);
/// }
/// ```
///
/// [maybe_async]: https://docs.rs/maybe-async/latest/maybe_async/index.html#macros-in-detail
#[proc_macro_attribute]
pub fn maybe_async(args: TokenStream, item: TokenStream) -> TokenStream {
    runtime::maybe_async(args.into(), item.into()).into()
}

/// Marks async only implementation.
///
/// When one of async runtime feature flags is set, this is replaced with
/// [`#[maybe_async::must_be_async]`][maybe_async]. The attribute arguments are passed
/// to the macro as they are.
///
/// Otherwise, the item attached by the attribute is removed.
///
/// # Examples
///
/// ```
/// # use odpi_rs_procmacro as odpi_rs;
/// # use std::task::{Context, Poll};
/// # use std::pin::Pin;
/// # pub trait AsyncIterator { // AsyncIterator is nightly.
/// #     type Item;
/// #     fn poll_next(
/// #         self: Pin<&mut Self>,
/// #         cx: &mut Context<'_>,
/// #     ) -> Poll<Option<Self::Item>>;
/// # }
/// struct Counter {
///     count: usize,
/// }
///
/// #[odpi_rs::async_impl]
/// impl AsyncIterator for Counter {
///     type Item = usize;
///     fn poll_next(
///         mut self: Pin<&mut Self>,
///         cx: &mut Context<'_>,
///     ) -> Poll<Option<Self::Item>> {
///         self.count += 1;
///         if self.count < 6 {
///             Poll::Ready(Some(self.count))
///         } else {
///             Poll::Ready(None)
///         }
///     }
/// }
/// ```
/// The above code is based on the [documentation for async_iter](https://doc.rust-lang.org/std/async_iter/index.html).
///
/// Equivalent code when one of async feature flags is set
///
/// ```
/// # use std::task::{Context, Poll};
/// # use std::pin::Pin;
/// # pub trait AsyncIterator { // AsyncIterator is nightly.
/// #     type Item;
/// #     fn poll_next(
/// #         self: Pin<&mut Self>,
/// #         cx: &mut Context<'_>,
/// #     ) -> Poll<Option<Self::Item>>;
/// # }
/// // features = ["tokio"] or another async runtime
/// struct Counter {
///     count: usize,
/// }
///
/// impl AsyncIterator for Counter {
///     type Item = usize;
///     fn poll_next(
///         mut self: Pin<&mut Self>,
///         cx: &mut Context<'_>,
///     ) -> Poll<Option<Self::Item>> {
///         self.count += 1;
///         if self.count < 6 {
///             Poll::Ready(Some(self.count))
///         } else {
///             Poll::Ready(None)
///         }
///     }
/// }
/// ```
///
/// Equivalent code when none of async feature flags are set
///
/// ```
/// // features = []
/// struct Counter {
///     count: usize,
/// }
///
/// // impl AsyncIterator for Counter { .. } is removed.
/// ```
///
/// [maybe_async]: https://docs.rs/maybe-async/latest/maybe_async/index.html#macros-in-detail
#[proc_macro_attribute]
pub fn async_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    runtime::async_impl(args.into(), item.into()).into()
}

/// Marks sync only implementation.
///
/// When none of async runtime feature flags are set, this is replaced with
/// [`#[maybe_async::must_be_sync]`][maybe_async].
///
/// Otherwise, the item marked by the attribute is removed.
///
/// # Examples
///
/// ```
/// # use odpi_rs_procmacro as odpi_rs;
/// struct Counter {
///     count: usize,
/// }
///
/// #[odpi_rs::sync_impl]
/// impl Iterator for Counter {
///     type Item = usize;
///     fn next(&mut self) -> Option<Self::Item> {
///         self.count += 1;
///         if self.count < 6 {
///             Some(self.count)
///         } else {
///             None
///         }
///     }
/// }
/// ```
/// The above code is based on the [documentation for iter](https://doc.rust-lang.org/std/iter/index.html).
///
/// Equivalent code when one of async feature flags is set
///
/// ```
/// // features = ["tokio"] or another async runtime
/// struct Counter {
///     count: usize,
/// }
///
/// // impl Iterator for Counter { .. } is removed.
/// ```
///
/// Equivalent code when none of async feature flags are set
///
/// ```
/// // features = []
/// struct Counter {
///     count: usize,
/// }
///
/// impl Iterator for Counter {
///     type Item = usize;
///     fn next(&mut self) -> Option<Self::Item> {
///         self.count += 1;
///         if self.count < 6 {
///             Some(self.count)
///         } else {
///             None
///         }
///     }
/// }
/// ```
///
/// [maybe_async]: https://docs.rs/maybe-async/latest/maybe_async/index.html#macros-in-detail
#[proc_macro_attribute]
pub fn sync_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    runtime::sync_impl(args.into(), item.into()).into()
}

/// Marks async function to be executed by the selected async runtime.
///
/// When one of async runtime feature flags is set, the marked code
/// is passed to the `block_on` function provided by the runtime.
///
/// Otherwise, the marked code is converted to sync with the help
/// of [`#[maybe_async::must_be_sync]`][maybe_async].
///
/// # Examples
///
/// ```
/// # use odpi_rs_procmacro as odpi_rs;
/// # mod tokio { pub mod runtime {
/// #     use std::future::Future;
/// #     pub struct Builder();
/// #     impl Builder {
/// #         pub fn new_multi_thread() -> Self { Builder() }
/// #         pub fn enable_all(&self) -> &Self { self }
/// #         pub fn build(&self) -> Result<Runtime, ()> { Ok(Runtime()) }
/// #     }
/// #     pub struct Runtime();
/// #     impl Runtime {
/// #         pub fn block_on<F: Future<Output = ()>>(&self, future: F) -> F::Output {
/// #             ()
/// #         }
/// #     }
/// # }}
/// # mod async_std { pub mod task {
/// #     use std::future::Future;
/// #     pub fn block_on<F: Future<Output = ()>>(future: F) -> F::Output {
/// #         ()
/// #     }
/// # }}
/// # mod smol {
/// #     use std::future::Future;
/// #     pub fn block_on<F: Future<Output = ()>>(future: F) -> F::Output {
/// #         ()
/// #     }
/// # }
/// # #[odpi_rs::maybe_async]
/// # async fn async_fn() -> bool { true }
/// #[odpi_rs::main]
/// async fn main() {
///     let res = async_fn().await;
///     assert!(res);
/// }
/// ```
///
/// Equivalent code when the `tokio` feature flag is set
///
/// ```
/// # mod tokio { pub mod runtime {
/// #     use std::future::Future;
/// #     pub struct Builder();
/// #     impl Builder {
/// #         pub fn new_multi_thread() -> Self { Builder() }
/// #         pub fn enable_all(&self) -> &Self { self }
/// #         pub fn build(&self) -> Result<Runtime, ()> { Ok(Runtime()) }
/// #     }
/// #     pub struct Runtime();
/// #     impl Runtime {
/// #         pub fn block_on<F: Future<Output = ()>>(&self, future: F) -> F::Output {
/// #             ()
/// #         }
/// #     }
/// # }}
/// # async fn async_fn() -> bool { true }
/// // features = ["tokio"]
/// fn main() {
///     tokio::runtime::Builder::new_multi_thread()
///         .enable_all()
///         .build()
///         .unwrap()
///         .block_on(async {
///             let res = async_fn().await;
///             assert!(res);
///         })
/// }
/// ```
///
/// Equivalent code when the `async-std` feature flag is set
///
/// ```
/// # mod async_std { pub mod task {
/// #     use std::future::Future;
/// #     pub fn block_on<F: Future<Output = ()>>(future: F) -> F::Output {
/// #         ()
/// #     }
/// # }}
/// # async fn async_fn() -> bool { true }
/// // features = ["async-std"]
/// fn main() {
///     async_std::task::block_on(async {
///         let res = async_fn().await;
///         assert!(res);
///     })
/// }
/// ```
///
/// Equivalent code when the `smol` feature flag is set
///
/// ```
/// # mod smol {
/// #     use std::future::Future;
/// #     pub fn block_on<F: Future<Output = ()>>(future: F) -> F::Output {
/// #         ()
/// #     }
/// # }
/// # async fn async_fn() -> bool { true }
/// // features = ["smol"]
/// fn main() {
///     smol::block_on(async {
///         let res = async_fn().await;
///         assert!(res);
///     })
/// }
/// ```
///
/// Equivalent code when none of async feature flags are set
///
/// ```
/// # fn async_fn() -> bool { true }
/// // features = []
/// fn main() {
///     let res = async_fn();
///     assert!(res);
/// }
/// ```
///
/// [maybe_async]: https://docs.rs/maybe-async/latest/maybe_async/index.html#macros-in-detail
#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    runtime::main(args.into(), item.into())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Marks async function to be executed by the selected async runtime, suitable to test environment.
///
/// When one of async runtime feature flags is set, the marked code
/// is passed to the `block_on` function provided by the runtime.
///
/// Otherwise, the code is converted to sync with the help
/// of [`#[maybe_async::must_be_sync]`][maybe_async].
///
/// # Examples
///
/// ```
/// # use odpi_rs_procmacro as odpi_rs;
/// # mod tokio { pub mod runtime {
/// #     use std::future::Future;
/// #     pub struct Builder();
/// #     impl Builder {
/// #         pub fn new_multi_thread() -> Self { Builder() }
/// #         pub fn enable_all(&self) -> &Self { self }
/// #         pub fn build(&self) -> Result<Runtime, ()> { Ok(Runtime()) }
/// #     }
/// #     pub struct Runtime();
/// #     impl Runtime {
/// #         pub fn block_on<F: Future<Output = ()>>(&self, future: F) -> F::Output {
/// #             ()
/// #         }
/// #     }
/// # }}
/// # mod async_std { pub mod task {
/// #     use std::future::Future;
/// #     pub fn block_on<F: Future<Output = ()>>(future: F) -> F::Output {
/// #         ()
/// #     }
/// # }}
/// # mod smol {
/// #     use std::future::Future;
/// #     pub fn block_on<F: Future<Output = ()>>(future: F) -> F::Output {
/// #         ()
/// #     }
/// # }
/// # #[odpi_rs::maybe_async]
/// # async fn async_fn() -> bool { true }
/// #[odpi_rs::test]
/// async fn my_test() {
///     let res = async_fn().await;
///     assert!(res);
/// }
/// ```
///
/// Equivalent code when the `tokio` feature flag is set
///
/// ```
/// # mod tokio { pub mod runtime {
/// #     use std::future::Future;
/// #     pub struct Builder();
/// #     impl Builder {
/// #         pub fn new_multi_thread() -> Self { Builder() }
/// #         pub fn enable_all(&self) -> &Self { self }
/// #         pub fn build(&self) -> Result<Runtime, ()> { Ok(Runtime()) }
/// #     }
/// #     pub struct Runtime();
/// #     impl Runtime {
/// #         pub fn block_on<F: Future<Output = ()>>(&self, future: F) -> F::Output {
/// #             ()
/// #         }
/// #     }
/// # }}
/// # async fn async_fn() -> bool { true }
/// // features = ["tokio"]
/// #[test]
/// fn my_test() {
///     tokio::runtime::Builder::new_multi_thread()
///         .enable_all()
///         .build()
///         .unwrap()
///         .block_on(async {
///             let res = async_fn().await;
///             assert!(res);
///         })
/// }
/// ```
///
/// Equivalent code when the `async-std` feature flag is set
///
/// ```
/// # mod async_std { pub mod task {
/// #     use std::future::Future;
/// #     pub fn block_on<F: Future<Output = ()>>(future: F) -> F::Output {
/// #         ()
/// #     }
/// # }}
/// # async fn async_fn() -> bool { true }
/// // features = ["async-std"]
/// #[test]
/// fn my_test() {
///     async_std::task::block_on(async {
///         let res = async_fn().await;
///         assert!(res);
///     })
/// }
/// ```
///
/// Equivalent code when the `smol` feature flag is set
///
/// ```
/// # mod smol {
/// #     use std::future::Future;
/// #     pub fn block_on<F: Future<Output = ()>>(future: F) -> F::Output {
/// #         ()
/// #     }
/// # }
/// # async fn async_fn() -> bool { true }
/// // features = ["smol"]
/// #[test]
/// fn my_test() {
///     smol::block_on(async {
///         let res = async_fn().await;
///         assert!(res);
///     })
/// }
/// ```
///
/// Equivalent code when none of async feature flags are set
///
/// ```
/// # fn async_fn() -> bool { true }
/// // features = []
/// #[test]
/// fn my_test() {
///     let res = async_fn();
///     assert!(res);
/// }
/// ```
///
/// [maybe_async]: https://docs.rs/maybe-async/latest/maybe_async/index.html#macros-in-detail
#[proc_macro_attribute]
pub fn test(args: TokenStream, item: TokenStream) -> TokenStream {
    runtime::test(args.into(), item.into())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

static ODPIC_DOC: LazyLock<OdpicDoc> = LazyLock::new(|| OdpicDoc::read_yaml().unwrap());

#[proc_macro_attribute]
pub fn odpic_doc(args: TokenStream, input: TokenStream) -> TokenStream {
    odpic_doc::odpic_doc(args, input)
}
