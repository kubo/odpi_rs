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
use crate::types::{Annotation, NativeType, ObjectType, OracleType, VectorFlags, VectorFormat};
use crate::utils::*;
use crate::*;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug, Clone)]
#[non_exhaustive]
#[odpic_doc]
pub struct DataTypeInfo {
    pub oracle_type: Option<OracleType>,
    pub default_native_type: Option<NativeType>,
    pub oci_type_code: u16,
    pub db_size_in_bytes: u32,
    pub client_size_in_bytes: u32,
    pub size_in_chars: u32,
    pub precision: i16,
    pub scale: i8,
    pub fs_precision: u8,
    pub object_type: Option<ObjectType>,
    pub is_json: bool,
    pub domain_schema: Option<String>,
    pub domain_name: Option<String>,
    pub annotations: Vec<Annotation>,
    pub is_oson: bool,
    pub vector_dimensions: u32,
    pub vector_format: Option<VectorFormat>,
    pub vector_flags: VectorFlags,
}

impl DataTypeInfo {
    pub(crate) fn from_dpi(info: &dpiDataTypeInfo) -> Result<DataTypeInfo> {
        Ok(DataTypeInfo {
            oracle_type: if info.oracleTypeNum != 0 {
                Some(info.oracleTypeNum.try_to_rust()?)
            } else {
                None
            },
            default_native_type: if info.defaultNativeTypeNum != 0 {
                Some(info.defaultNativeTypeNum.try_to_rust()?)
            } else {
                None
            },
            oci_type_code: info.ociTypeCode,
            db_size_in_bytes: info.dbSizeInBytes,
            client_size_in_bytes: info.clientSizeInBytes,
            size_in_chars: info.sizeInChars,
            precision: info.precision,
            scale: info.scale,
            fs_precision: info.fsPrecision,
            object_type: ObjectType::with_add_ref(info.objectType),
            is_json: info.isJson.to_rust(),
            domain_schema: (info.domainSchema, info.domainSchemaLength).try_to_rust()?,
            domain_name: (info.domainName, info.domainNameLength).try_to_rust()?,
            annotations: (0..info.numAnnotations)
                .map(|idx| Annotation::from_dpi(unsafe { &*info.annotations.offset(idx as isize) }))
                .collect::<Result<Vec<_>>>()?,
            is_oson: info.isOson.to_rust(),
            vector_dimensions: info.vectorDimensions,
            vector_format: if info.vectorFormat != 0 {
                Some(info.vectorFormat.try_to_rust()?)
            } else {
                None
            },
            vector_flags: info.vectorFlags.to_rust(),
        })
    }
}

impl TryToRust<DataTypeInfo> for dpiDataTypeInfo {
    fn try_to_rust(&self) -> Result<DataTypeInfo> {
        DataTypeInfo::from_dpi(self)
    }
}
