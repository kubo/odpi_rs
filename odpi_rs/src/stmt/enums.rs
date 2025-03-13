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
    pub struct ExecMode: dpiExecMode {
        #[odpic_doc]
        const ARRAY_DML_ROWCOUNTS = DPI_MODE_EXEC_ARRAY_DML_ROWCOUNTS;
        #[odpic_doc]
        const BATCH_ERRORS = DPI_MODE_EXEC_BATCH_ERRORS;
        #[odpic_doc]
        const COMMIT_ON_SUCCESS = DPI_MODE_EXEC_COMMIT_ON_SUCCESS;
        #[odpic_doc]
        const DEFAULT = DPI_MODE_EXEC_DEFAULT;
        #[odpic_doc]
        const DESCRIBE_ONLY = DPI_MODE_EXEC_DESCRIBE_ONLY;
        #[odpic_doc]
        const PARSE_ONLY = DPI_MODE_EXEC_PARSE_ONLY;
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    #[repr(u16)]
    pub enum FetchMode: dpiFetchMode {
        Absolute = DPI_MODE_FETCH_ABSOLUTE,
        First = DPI_MODE_FETCH_FIRST,
        Last = DPI_MODE_FETCH_LAST,
        Next = DPI_MODE_FETCH_NEXT,
        Priorn = DPI_MODE_FETCH_PRIOR,
        Relative = DPI_MODE_FETCH_RELATIVE,
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    #[repr(u16)]
    pub enum StatementType: dpiStatementType {
        Unknown = DPI_STMT_TYPE_UNKNOWN,
        Select = DPI_STMT_TYPE_SELECT,
        Update = DPI_STMT_TYPE_UPDATE,
        Delete = DPI_STMT_TYPE_DELETE,
        Insert = DPI_STMT_TYPE_INSERT,
        Create = DPI_STMT_TYPE_CREATE,
        Drop = DPI_STMT_TYPE_DROP,
        Alter = DPI_STMT_TYPE_ALTER,
        Begin = DPI_STMT_TYPE_BEGIN,
        Declare = DPI_STMT_TYPE_DECLARE,
        Call = DPI_STMT_TYPE_CALL,
        ExplainPlan = DPI_STMT_TYPE_EXPLAIN_PLAN,
        Merge = DPI_STMT_TYPE_MERGE,
        Rollback = DPI_STMT_TYPE_ROLLBACK,
        Commit = DPI_STMT_TYPE_COMMIT,
    }
}
