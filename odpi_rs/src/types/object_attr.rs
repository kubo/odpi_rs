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
