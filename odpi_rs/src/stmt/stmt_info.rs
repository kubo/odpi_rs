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
use crate::stmt::StatementType;
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug, Clone)]
#[non_exhaustive]
#[odpic_doc]
pub struct StmtInfo {
    pub is_query: bool,
    pub is_plsql: bool,
    pub is_ddl: bool,
    pub is_dml: bool,
    pub statement_type: StatementType,
    pub is_returning: bool,
}

impl TryToRust<StmtInfo> for dpiStmtInfo {
    fn try_to_rust(&self) -> Result<StmtInfo> {
        Ok(StmtInfo {
            is_query: self.isQuery.to_rust(),
            is_plsql: self.isPLSQL.to_rust(),
            is_ddl: self.isDDL.to_rust(),
            is_dml: self.isDML.to_rust(),
            statement_type: self.statementType.try_to_rust()?,
            is_returning: self.isReturning.to_rust(),
        })
    }
}
