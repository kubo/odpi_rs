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
use std::ffi::c_char;
use std::ptr;

#[derive(Clone, Debug)]
#[odpic_doc]
///
/// This enum has only one variant `Varchar` currently because
/// [`dpiShardingKeyColumn::oracleTypeNum`] supports `DPI_ORACLE_TYPE_VARCHAR` only at present.
#[non_exhaustive]
pub enum ShardingKeyColumn {
    Varchar(String),
}

impl TryToDpi<dpiShardingKeyColumn> for ShardingKeyColumn {
    fn try_to_dpi(&self) -> Result<dpiShardingKeyColumn> {
        match self {
            ShardingKeyColumn::Varchar(key) => Ok(dpiShardingKeyColumn {
                oracleTypeNum: DPI_ORACLE_TYPE_VARCHAR,
                nativeTypeNum: DPI_NATIVE_TYPE_BYTES,
                value: dpiDataBuffer {
                    asBytes: dpiBytes {
                        ptr: key.as_ptr() as *mut c_char,
                        length: key.len().try_into()?,
                        encoding: ptr::null(),
                    },
                },
            }),
        }
    }
}
