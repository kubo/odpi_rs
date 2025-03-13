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
use crate::*;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::borrow::Cow;
use std::ffi::c_long;

#[derive(Debug, Clone)]
#[non_exhaustive]
#[odpic_doc]
pub struct Xid<'a> {
    pub format_id: c_long,
    pub global_transaction_id: Cow<'a, [u8]>,
    pub branch_qualifier: Cow<'a, [u8]>,
}

impl ToDpi<dpiXid> for Xid<'_> {
    fn to_dpi(&self) -> dpiXid {
        let (gt, gt_len) = self.global_transaction_id.as_ref().to_dpi();
        let (bq, bq_len) = self.branch_qualifier.as_ref().to_dpi();
        dpiXid {
            formatId: self.format_id,
            globalTransactionId: gt,
            globalTransactionIdLength: gt_len,
            branchQualifier: bq,
            branchQualifierLength: bq_len,
        }
    }
}
