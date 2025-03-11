use crate::maybe_async;
use crate::soda::Coll;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug)]
#[odpic_doc(name = "dpiSodaCollCursor")]
pub struct CollCursor {
    pub(crate) handle: *mut dpiSodaCollCursor,
}

#[odpic_doc(name = "dpiSodaCollCursor")]
impl CollCursor {
    pub(crate) fn new(handle: *mut dpiSodaCollCursor) -> CollCursor {
        CollCursor { handle }
    }

    pub fn close(&self) -> Result<()> {
        call!(dpiSodaCollCursor_close(self.handle))
    }

    #[maybe_async]
    pub async fn next(&self, flags: u32) -> Result<Coll> {
        Ok(Coll::new(
            *get_value_blocking! {
                let handle = self.handle;
                dpiSodaCollCursor_getNext(*handle, flags)
            }
            .await?,
        ))
    }
}

impl Clone for CollCursor {
    fn clone(&self) -> CollCursor {
        unsafe { dpiSodaCollCursor_addRef(self.handle) };
        CollCursor {
            handle: self.handle,
        }
    }
}

impl Drop for CollCursor {
    fn drop(&mut self) {
        release_handle!(dpiSodaCollCursor_release(self.handle));
    }
}
