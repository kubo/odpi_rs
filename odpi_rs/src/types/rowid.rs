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
use crate::utils::*;
use crate::*;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug)]
#[odpic_doc]
pub struct Rowid {
    pub(crate) handle: *mut dpiRowid,
}

#[odpic_doc]
impl Rowid {
    pub(crate) fn with_add_ref(handle: *mut dpiRowid) -> Rowid {
        unsafe { dpiRowid_addRef(handle) };
        Rowid { handle }
    }

    pub fn string_value(&self) -> Result<&str> {
        let (ptr, len) = get_2values!(dpiRowid_getStringValue(self.handle))?;
        (ptr, len).try_to_rust()
    }
}

impl Clone for Rowid {
    fn clone(&self) -> Rowid {
        unsafe { dpiRowid_addRef(self.handle) };
        Rowid {
            handle: self.handle,
        }
    }
}

impl Drop for Rowid {
    fn drop(&mut self) {
        unsafe { dpiRowid_release(self.handle) };
    }
}
