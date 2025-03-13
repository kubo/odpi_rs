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

#[derive(Clone, Debug)]
#[odpic_doc]
pub struct AccessToken {
    pub token: Vec<u8>,
    pub private_key: Vec<u8>,
}

impl AccessToken {
    pub fn new<T, P>(token: T, private_key: P) -> AccessToken
    where
        T: Into<Vec<u8>>,
        P: Into<Vec<u8>>,
    {
        AccessToken {
            token: token.into(),
            private_key: private_key.into(),
        }
    }
}

impl TryToDpi<dpiAccessToken> for AccessToken {
    fn try_to_dpi(&self) -> Result<dpiAccessToken> {
        Ok(dpiAccessToken {
            token: self.token.as_ptr() as *const c_char,
            tokenLength: self.token.len().try_into()?,
            privateKey: self.private_key.as_ptr() as *const c_char,
            privateKeyLength: self.private_key.len().try_into()?,
        })
    }
}
