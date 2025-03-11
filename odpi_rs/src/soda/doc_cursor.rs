use crate::maybe_async;
use crate::soda::Doc;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug)]
#[odpic_doc(name = "dpiSodaDocCursor")]
pub struct DocCursor {
    pub(crate) handle: *mut dpiSodaDocCursor,
}

#[odpic_doc(name = "dpiSodaDocCursor")]
impl DocCursor {
    pub(crate) fn new(handle: *mut dpiSodaDocCursor) -> DocCursor {
        DocCursor { handle }
    }

    pub fn close(&self) -> Result<()> {
        call!(dpiSodaDocCursor_close(self.handle))
    }

    #[maybe_async]
    pub async fn next(&self, flags: u32) -> Result<Doc> {
        Ok(Doc::new(
            *get_value_blocking! {
                let handle = self.handle;
                dpiSodaDocCursor_getNext(*handle, flags)
            }
            .await?,
        ))
    }
}

impl Clone for DocCursor {
    fn clone(&self) -> DocCursor {
        unsafe { dpiSodaDocCursor_addRef(self.handle) };
        DocCursor {
            handle: self.handle,
        }
    }
}

impl Drop for DocCursor {
    fn drop(&mut self) {
        release_handle!(dpiSodaDocCursor_release(self.handle));
    }
}
