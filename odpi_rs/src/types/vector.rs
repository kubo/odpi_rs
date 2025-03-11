use crate::types::VectorInfo;
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug)]
#[odpic_doc]
pub struct Vector {
    pub(crate) handle: *mut dpiVector,
}

impl Vector {
    pub(crate) fn new(handle: *mut dpiVector) -> Vector {
        Vector { handle }
    }

    pub(crate) fn with_add_ref(handle: *mut dpiVector) -> Vector {
        unsafe { dpiVector_addRef(handle) };
        Vector { handle }
    }

    pub fn value(&self) -> Result<VectorInfo> {
        get_value!(dpiVector_getValue(self.handle))?.try_to_rust()
    }

    pub fn set_value(&self, info: &VectorInfo<'_>) -> Result<()> {
        call!(dpiVector_setValue(self.handle, &mut info.to_dpi()))
    }
}

impl Clone for Vector {
    fn clone(&self) -> Vector {
        unsafe { dpiVector_addRef(self.handle) };
        Vector {
            handle: self.handle,
        }
    }
}

impl Drop for Vector {
    fn drop(&mut self) {
        unsafe { dpiVector_release(self.handle) };
    }
}
