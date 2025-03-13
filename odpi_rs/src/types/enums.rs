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

dpi_bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    pub struct JsonOptions: u32 {
        #[odpic_doc]
        const DATE_AS_DOUBLE = DPI_JSON_OPT_DATE_AS_DOUBLE;
        #[odpic_doc]
        const DEFAULT = DPI_JSON_OPT_DEFAULT;
        #[odpic_doc]
        const NUMBER_AS_STRING = DPI_JSON_OPT_NUMBER_AS_STRING;
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    #[odpic_doc(name = "dpiNativeTypeNum")]
    #[repr(u32)]
    pub enum NativeType: dpiNativeTypeNum {
        Int64 = DPI_NATIVE_TYPE_INT64,
        Uint64 = DPI_NATIVE_TYPE_UINT64,
        Float = DPI_NATIVE_TYPE_FLOAT,
        Double = DPI_NATIVE_TYPE_DOUBLE,
        Bytes = DPI_NATIVE_TYPE_BYTES,
        Timestamp = DPI_NATIVE_TYPE_TIMESTAMP,
        IntervalDS = DPI_NATIVE_TYPE_INTERVAL_DS,
        IntervalYM = DPI_NATIVE_TYPE_INTERVAL_YM,
        Lob = DPI_NATIVE_TYPE_LOB,
        Object = DPI_NATIVE_TYPE_OBJECT,
        Stmt = DPI_NATIVE_TYPE_STMT,
        Boolean = DPI_NATIVE_TYPE_BOOLEAN,
        Rowid = DPI_NATIVE_TYPE_ROWID,
        Json = DPI_NATIVE_TYPE_JSON,
        JsonObject = DPI_NATIVE_TYPE_JSON_OBJECT,
        JsonArray = DPI_NATIVE_TYPE_JSON_ARRAY,
        Null = DPI_NATIVE_TYPE_NULL,
        Vector = DPI_NATIVE_TYPE_VECTOR,
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    #[odpic_doc(name = "dpiOracleTypeNum")]
    #[repr(u32)]
    pub enum OracleType: dpiOracleTypeNum {
        None = DPI_ORACLE_TYPE_NONE,
        Varchar = DPI_ORACLE_TYPE_VARCHAR,
        Nvarchar = DPI_ORACLE_TYPE_NVARCHAR,
        Char = DPI_ORACLE_TYPE_CHAR,
        Nchar = DPI_ORACLE_TYPE_NCHAR,
        Rowid = DPI_ORACLE_TYPE_ROWID,
        Raw = DPI_ORACLE_TYPE_RAW,
        NativeFloat = DPI_ORACLE_TYPE_NATIVE_FLOAT,
        NativeDouble = DPI_ORACLE_TYPE_NATIVE_DOUBLE,
        NativeInt = DPI_ORACLE_TYPE_NATIVE_INT,
        Number = DPI_ORACLE_TYPE_NUMBER,
        Date = DPI_ORACLE_TYPE_DATE,
        Timestamp = DPI_ORACLE_TYPE_TIMESTAMP,
        TimestampTZ = DPI_ORACLE_TYPE_TIMESTAMP_TZ,
        TimestampLTZ = DPI_ORACLE_TYPE_TIMESTAMP_LTZ,
        IntervalDS = DPI_ORACLE_TYPE_INTERVAL_DS,
        IntervalYM = DPI_ORACLE_TYPE_INTERVAL_YM,
        Clob = DPI_ORACLE_TYPE_CLOB,
        Nclob = DPI_ORACLE_TYPE_NCLOB,
        Blob = DPI_ORACLE_TYPE_BLOB,
        Bfile = DPI_ORACLE_TYPE_BFILE,
        Stmt = DPI_ORACLE_TYPE_STMT,
        Boolean = DPI_ORACLE_TYPE_BOOLEAN,
        Object = DPI_ORACLE_TYPE_OBJECT,
        LongVarchar = DPI_ORACLE_TYPE_LONG_VARCHAR,
        LongRaw = DPI_ORACLE_TYPE_LONG_RAW,
        NativeUint = DPI_ORACLE_TYPE_NATIVE_UINT,
        Json = DPI_ORACLE_TYPE_JSON,
        JsonObject = DPI_ORACLE_TYPE_JSON_OBJECT,
        JsonArray = DPI_ORACLE_TYPE_JSON_ARRAY,
        Urowid = DPI_ORACLE_TYPE_UROWID,
        LongNvarchar = DPI_ORACLE_TYPE_LONG_NVARCHAR,
        XmlType = DPI_ORACLE_TYPE_XMLTYPE,
        Vector = DPI_ORACLE_TYPE_VECTOR,
        JsonId = DPI_ORACLE_TYPE_JSON_ID,
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    #[odpic_doc]
    #[repr(u8)]
    pub enum VectorFormat: dpiVectorFormat {
        Binary = DPI_VECTOR_FORMAT_BINARY,
        Float32 = DPI_VECTOR_FORMAT_FLOAT32,
        Float64 = DPI_VECTOR_FORMAT_FLOAT64,
        Int8 = DPI_VECTOR_FORMAT_INT8,
    }
}

dpi_bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    pub struct VectorFlags: dpiVectorFlags {
        #[odpic_doc]
        const FLEXIBLE_DIM = DPI_VECTOR_FLAGS_FLEXIBLE_DIM;
    }
}
