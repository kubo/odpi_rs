use crate::aq::{MessageDeliveryMode, Visibility};
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug)]
#[odpic_doc]
pub struct EnqOptions {
    pub(crate) handle: *mut dpiEnqOptions,
}

#[odpic_doc]
impl EnqOptions {
    pub(crate) fn new(handle: *mut dpiEnqOptions) -> EnqOptions {
        EnqOptions { handle }
    }

    pub fn transformation(&self) -> Result<String> {
        get_string_value!(dpiEnqOptions_getTransformation(self.handle))
    }

    pub fn visibility(&self) -> Result<Visibility> {
        get_value!(dpiEnqOptions_getVisibility(self.handle))?.try_to_rust()
    }

    pub fn set_delivery_mode(&self, value: MessageDeliveryMode) -> Result<()> {
        call!(dpiEnqOptions_setDeliveryMode(self.handle, value.to_dpi()))
    }

    pub fn set_transformation<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        set_str_value!(dpiEnqOptions_setTransformation(self.handle, value))
    }

    pub fn set_visibility(&self, value: Visibility) -> Result<()> {
        call!(dpiEnqOptions_setVisibility(self.handle, value.to_dpi()))
    }
}

impl Clone for EnqOptions {
    fn clone(&self) -> EnqOptions {
        unsafe { dpiEnqOptions_addRef(self.handle) };
        EnqOptions {
            handle: self.handle,
        }
    }
}

impl Drop for EnqOptions {
    fn drop(&mut self) {
        release_handle!(dpiEnqOptions_release(self.handle));
    }
}
