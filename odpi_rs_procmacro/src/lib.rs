use odpic_sys::doc::OdpicDoc;
use proc_macro::TokenStream;
use std::sync::LazyLock;
use syn::Error;

mod attr_macro;
mod odpic_doc;

enum AsyncRuntime {
    Tokio,
    AsyncStd,
    Smol,
    None,
}

impl AsyncRuntime {
    const fn is_sync(&self) -> bool {
        matches!(self, AsyncRuntime::None)
    }
    const fn is_async(&self) -> bool {
        !self.is_sync()
    }
}

const RT: AsyncRuntime = if cfg!(feature = "tokio") {
    AsyncRuntime::Tokio
} else if cfg!(feature = "async-std") {
    AsyncRuntime::AsyncStd
} else if cfg!(feature = "smol") {
    AsyncRuntime::Smol
} else {
    AsyncRuntime::None
};

#[proc_macro_attribute]
/// The original `maybe_async` macro keeps async code when "is_sync" is not set
/// and converts async code to sync when "is_sync" is set.
/// This `maybe_async` macro keeps async code when "is_async" is set
/// and converts async code to sync when "is_async" is not set.
pub fn maybe_async(args: TokenStream, item: TokenStream) -> TokenStream {
    attr_macro::maybe_async(args.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    attr_macro::main(args.into(), item.into())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn test(args: TokenStream, item: TokenStream) -> TokenStream {
    attr_macro::test(args.into(), item.into())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

static ODPIC_DOC: LazyLock<OdpicDoc> = LazyLock::new(|| OdpicDoc::read_yaml().unwrap());

#[proc_macro_attribute]
pub fn odpic_doc(args: TokenStream, input: TokenStream) -> TokenStream {
    odpic_doc::odpic_doc(args, input)
}
