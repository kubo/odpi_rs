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
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Clone, Debug)]
#[odpic_doc]
pub struct AppContext {
    pub namespace_name: String,
    pub name: String,
    pub value: String,
}

impl AppContext {
    pub fn new<NS, N, V>(namespace_name: NS, name: N, value: V) -> AppContext
    where
        NS: Into<String>,
        N: Into<String>,
        V: Into<String>,
    {
        AppContext {
            namespace_name: namespace_name.into(),
            name: name.into(),
            value: value.into(),
        }
    }
}

impl TryToDpi<dpiAppContext> for AppContext {
    fn try_to_dpi(&self) -> Result<dpiAppContext> {
        Ok(dpiAppContext {
            namespaceName: self.namespace_name.to_ptr(),
            namespaceNameLength: self.namespace_name.try_to_len()?,
            name: self.name.to_ptr(),
            nameLength: self.name.try_to_len()?,
            value: self.value.to_ptr(),
            valueLength: self.value.try_to_len()?,
        })
    }
}
