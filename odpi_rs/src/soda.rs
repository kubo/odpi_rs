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
//! Simple Oracle Document Access (SODA)

mod coll;
mod coll_cursor;
mod db;
mod doc;
mod doc_cursor;
mod enums;
mod oper_options;

pub use coll::Coll;
pub use coll_cursor::CollCursor;
pub use db::Db;
pub use doc::Doc;
pub use doc_cursor::DocCursor;
pub use enums::*;
pub use oper_options::OperOptions;
