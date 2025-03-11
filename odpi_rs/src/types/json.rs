use crate::maybe_async;
use crate::types::JsonOptions;
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

pub mod array;
mod json_node;
mod native_value;
pub mod object;

pub use json_node::*;
pub use native_value::NativeValue;

#[derive(Debug)]
#[odpic_doc]
#[doc(hidden)] // re-export in crate::types with doc by #[doc(inline)]
pub struct Json {
    pub(crate) handle: *mut dpiJson,
}

unsafe impl Send for Json {}

impl Json {
    pub(crate) fn new(handle: *mut dpiJson) -> Json {
        Json { handle }
    }

    pub(crate) fn with_add_ref(handle: *mut dpiJson) -> Json {
        unsafe { dpiJson_addRef(handle) };
        Json { handle }
    }

    #[maybe_async]
    pub async unsafe fn unsafe_value(&self, options: JsonOptions) -> Result<&Node> {
        let top_node = *get_value_blocking! {
            let handle = self.handle;
            dpiJson_getValue(*handle, options.to_dpi())
        }
        .await?;
        Ok(Node::ref_from_dpi_ptr(top_node))
    }

    #[maybe_async]
    pub async fn value(&self, options: JsonOptions) -> Result<Value> {
        unsafe { self.unsafe_value(options).await? }.to_owned()
    }

    ///
    /// The `flags` arguemnt is one or more of [JSON Validation Constants], OR'ed together. `JZN_ALLOW_SCALAR_DOCUMENTS` is set by default.
    ///
    /// [JSON Validation Constants]: https://docs.oracle.com/en/database/oracle/oracle-database/23/caxml/JSON-DOM-interfaces.html#GUID-4B5C4646-8EA8-40D1-B341-919F1D8E9A57__GUID-8FF5678E-7896-43AB-8D56-10A1846C75AF
    ///
    pub fn set_from_text<T>(&self, value: T, flags: u32) -> Result<()>
    where
        T: AsRef<str>,
    {
        call!(dpiJson_setFromText(
            self.handle,
            value.to_ptr(),
            value.as_ref().len().try_into()?,
            flags
        ))
    }

    pub fn set_value<'a, T>(&self, top_node: T) -> Result<()>
    where
        T: Into<dpiJsonNode>,
    {
        call!(dpiJson_setValue(self.handle, &mut top_node.into()))
    }
}

impl Clone for Json {
    fn clone(&self) -> Json {
        unsafe { dpiJson_addRef(self.handle) };
        Json {
            handle: self.handle,
        }
    }
}

impl Drop for Json {
    fn drop(&mut self) {
        release_handle!(dpiJson_release(self.handle));
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_util, Result};

    #[crate::test(maybe_async)]
    async fn set_json_var() -> Result<()> {
        let conn = test_util::connect().await?;
        let _json = conn.new_json()?;
        Ok(())
    }
}
