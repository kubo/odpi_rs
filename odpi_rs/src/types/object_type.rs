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
use crate::types::{Object, ObjectAttr};
use crate::utils::*;
use crate::*;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::ptr;

#[derive(Debug)]
#[odpic_doc]
pub struct ObjectType {
    pub(crate) handle: *mut dpiObjectType,
}

impl ObjectType {
    pub(crate) fn new(handle: *mut dpiObjectType) -> ObjectType {
        ObjectType { handle }
    }

    pub(crate) fn with_add_ref(handle: *mut dpiObjectType) -> Option<ObjectType> {
        if handle.is_null() {
            None
        } else {
            unsafe { dpiObjectType_addRef(handle) };
            Some(ObjectType { handle })
        }
    }

    pub fn create_object(&self) -> Result<Object> {
        Ok(Object::new(get_value!(dpiObjectType_createObject(
            self.handle
        ))?))
    }

    pub fn attributes(&self, num_attributes: u16) -> Result<ObjectAttr> {
        Ok(ObjectAttr::new(get_value!(dpiObjectType_getAttributes(
            self.handle,
            num_attributes
        ))?))
    }

    // // return information about the object type
    // DPI_EXPORT int dpiObjectType_getInfo(dpiObjectType *objType,
    //         dpiObjectTypeInfo *info);
}

impl Clone for ObjectType {
    fn clone(&self) -> ObjectType {
        unsafe { dpiObjectType_addRef(self.handle) };
        ObjectType {
            handle: self.handle,
        }
    }
}

impl Drop for ObjectType {
    fn drop(&mut self) {
        release_handle!(dpiObjectType_release(self.handle));
    }
}

impl ToDpi<*mut dpiObjectType> for Option<&ObjectType> {
    fn to_dpi(&self) -> *mut dpiObjectType {
        self.map(|objtype| objtype.handle)
            .unwrap_or_else(ptr::null_mut)
    }
}
