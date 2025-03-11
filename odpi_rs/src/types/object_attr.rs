use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug)]
#[odpic_doc]
pub struct ObjectAttr {
    pub(crate) handle: *mut dpiObjectAttr,
}

impl ObjectAttr {
    pub(crate) fn new(handle: *mut dpiObjectAttr) -> ObjectAttr {
        ObjectAttr { handle }
    }
}

impl Clone for ObjectAttr {
    fn clone(&self) -> ObjectAttr {
        unsafe { dpiObjectAttr_addRef(self.handle) };
        ObjectAttr {
            handle: self.handle,
        }
    }
}

impl Drop for ObjectAttr {
    fn drop(&mut self) {
        release_handle!(dpiObjectAttr_release(self.handle));
    }
}
