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
use crate::types::DataTypeInfo;
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug, Clone)]
#[non_exhaustive]
#[odpic_doc]
pub struct QueryInfo {
    pub name: String,
    pub type_info: DataTypeInfo,
    pub null_ok: bool,
}

impl TryToRust<QueryInfo> for dpiQueryInfo {
    fn try_to_rust(&self) -> Result<QueryInfo> {
        Ok(QueryInfo {
            name: (self.name, self.nameLength).try_to_rust()?,
            type_info: self.typeInfo.try_to_rust()?,
            null_ok: self.nullOk.to_rust(),
        })
    }
}
