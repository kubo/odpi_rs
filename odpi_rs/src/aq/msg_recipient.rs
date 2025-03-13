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
use std::borrow::Cow;

#[derive(Debug, Clone)]
#[odpic_doc]
pub struct MsgRecipient<'a> {
    pub name: Cow<'a, str>,
}

impl MsgRecipient<'_> {
    pub fn new<'a, T>(name: T) -> MsgRecipient<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        MsgRecipient { name: name.into() }
    }
}

impl TryToDpi<dpiMsgRecipient> for MsgRecipient<'_> {
    fn try_to_dpi(&self) -> Result<dpiMsgRecipient> {
        Ok(dpiMsgRecipient {
            name: self.name.to_ptr(),
            nameLength: self.name.try_to_len()?,
        })
    }
}
