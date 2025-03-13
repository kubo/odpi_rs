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
//! Oracle data types and type information

mod annotation;
mod data_type_info;
mod enums;
mod from_sql;
mod interval_ds;
mod interval_ym;
pub mod json;
mod lob;
mod native_value;
mod object;
mod object_attr;
mod object_type;
mod rowid;
mod timestamp;
mod vector;
mod vector_info;
mod xid;

pub use annotation::Annotation;
pub use data_type_info::DataTypeInfo;
pub use enums::*;
pub use from_sql::FromSql;
pub use from_sql::FromSqlUnsafe;
pub use interval_ds::IntervalDS;
pub use interval_ym::IntervalYM;
#[doc(inline)]
pub use json::Json;
pub use lob::Lob;
pub use native_value::NativeValue;
pub use object::Object;
pub use object_attr::ObjectAttr;
pub use object_type::ObjectType;
pub use rowid::Rowid;
pub use timestamp::Timestamp;
pub use vector::Vector;
pub use vector_info::VectorInfo;
pub use xid::Xid;
