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
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::borrow::Cow;
use std::ffi::CStr;
use std::ptr;

const DRIVER_NAME: &CStr = unsafe {
    CStr::from_bytes_with_nul_unchecked(
        concat!(
            env!("CARGO_PKG_NAME"),
            " : ",
            env!("CARGO_PKG_VERSION"),
            "\0"
        )
        .as_bytes(),
    )
};

#[derive(Clone, Debug)]
#[non_exhaustive]
#[odpic_doc]
pub struct ContextCreateParams<'a> {
    pub default_driver_name: Option<Cow<'a, CStr>>,
    pub load_error_url: Option<Cow<'a, CStr>>,
    pub oracle_client_lib_dir: Option<Cow<'a, CStr>>,
    pub oracle_client_config_dir: Option<Cow<'a, CStr>>,
    pub soda_use_json_desc: bool,
    pub use_json_id: bool,
}

impl<'a> ContextCreateParams<'a> {
    pub fn new() -> ContextCreateParams<'static> {
        ContextCreateParams {
            default_driver_name: Some(DRIVER_NAME.into()),
            load_error_url: None,
            oracle_client_lib_dir: None,
            oracle_client_config_dir: None,
            soda_use_json_desc: false,
            use_json_id: false,
        }
    }

    pub(crate) fn to_dpi(&self) -> dpiContextCreateParams {
        dpiContextCreateParams {
            defaultDriverName: self.default_driver_name.to_dpi(),
            defaultEncoding: ptr::null(),
            loadErrorUrl: self.load_error_url.to_dpi(),
            oracleClientLibDir: self.oracle_client_lib_dir.to_dpi(),
            oracleClientConfigDir: self.oracle_client_config_dir.to_dpi(),
            sodaUseJsonDesc: self.soda_use_json_desc.to_dpi(),
            useJsonId: self.use_json_id.to_dpi(),
        }
    }

    pub(crate) fn set_dpi_params(&mut self, sys: &dpiContextCreateParams) {
        self.soda_use_json_desc = sys.sodaUseJsonDesc != 0;
    }

    /// Sets `value` to [`field@ContextCreateParams::default_driver_name`] field.
    pub fn default_driver_name<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Cow<'a, CStr>>,
    {
        self.default_driver_name = Some(value.into());
        self
    }

    /// Sets `value` to [`field@ContextCreateParams::load_error_url`] field.
    pub fn load_error_url<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Cow<'a, CStr>>,
    {
        self.load_error_url = Some(value.into());
        self
    }

    /// Sets `value` to [`field@ContextCreateParams::oracle_client_lib_dir`] field.
    pub fn oracle_client_lib_dir<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Cow<'a, CStr>>,
    {
        self.oracle_client_lib_dir = Some(value.into());
        self
    }

    /// Sets `value` to [`field@ContextCreateParams::oracle_client_config_dir`] field.
    pub fn oracle_client_config_dir<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Cow<'a, CStr>>,
    {
        self.oracle_client_config_dir = Some(value.into());
        self
    }

    /// Sets `value` to [`field@ContextCreateParams::soda_use_json_desc`] field.
    pub fn soda_use_json_desc(&mut self, value: bool) -> &mut Self {
        self.soda_use_json_desc = value;
        self
    }

    /// Sets `value` to [`field@ContextCreateParams::use_json_id`] field.
    pub fn use_json_id(&mut self, value: bool) -> &mut Self {
        self.use_json_id = value;
        self
    }
}

impl Default for ContextCreateParams<'static> {
    fn default() -> ContextCreateParams<'static> {
        ContextCreateParams::new()
    }
}
