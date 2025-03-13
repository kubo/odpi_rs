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
