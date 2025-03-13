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
