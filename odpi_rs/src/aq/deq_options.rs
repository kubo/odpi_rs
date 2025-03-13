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
use crate::aq::{DeqMode, DeqNavigation, MessageDeliveryMode, Visibility};
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::time::Duration;

#[derive(Debug)]
#[odpic_doc]
pub struct DeqOptions {
    pub(crate) handle: *mut dpiDeqOptions,
}

#[odpic_doc]
impl DeqOptions {
    pub fn condition(&self) -> Result<String> {
        get_string_value!(dpiDeqOptions_getCondition(self.handle))
    }

    pub fn consumer_name(&self) -> Result<String> {
        get_string_value!(dpiDeqOptions_getConsumerName(self.handle))
    }

    pub fn correlation(&self) -> Result<String> {
        get_string_value!(dpiDeqOptions_getCorrelation(self.handle))
    }

    pub fn mode(&self) -> Result<DeqMode> {
        get_value!(dpiDeqOptions_getMode(self.handle))?.try_to_rust()
    }

    pub fn msg_id(&self) -> Result<String> {
        get_string_value!(dpiDeqOptions_getMsgId(self.handle))
    }

    pub fn navigation(&self) -> Result<DeqNavigation> {
        get_value!(dpiDeqOptions_getNavigation(self.handle))?.try_to_rust()
    }

    pub fn transformation(&self) -> Result<String> {
        get_string_value!(dpiDeqOptions_getTransformation(self.handle))
    }

    pub fn visibility(&self) -> Result<Visibility> {
        get_value!(dpiDeqOptions_getVisibility(self.handle))?.try_to_rust()
    }

    pub fn wait(&self) -> Result<Duration> {
        Ok(Duration::from_secs(
            get_value!(dpiDeqOptions_getWait(self.handle))?.into(),
        ))
    }

    pub fn set_condition<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        set_str_value!(dpiDeqOptions_setCondition(self.handle, value))
    }

    pub fn set_consumer_name<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        set_str_value!(dpiDeqOptions_setConsumerName(self.handle, value))
    }

    pub fn set_correlation<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        set_str_value!(dpiDeqOptions_setCorrelation(self.handle, value))
    }

    pub fn set_delivery_mode(&self, value: MessageDeliveryMode) -> Result<()> {
        call!(dpiDeqOptions_setDeliveryMode(self.handle, value.to_dpi()))
    }

    pub fn set_mode(&self, value: DeqMode) -> Result<()> {
        call!(dpiDeqOptions_setMode(self.handle, value.to_dpi()))
    }

    pub fn set_msg_id<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        set_str_value!(dpiDeqOptions_setMsgId(self.handle, value))
    }

    pub fn set_navigation(&self, value: DeqNavigation) -> Result<()> {
        call!(dpiDeqOptions_setNavigation(self.handle, value.to_dpi()))
    }

    pub fn set_transformation<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        set_str_value!(dpiDeqOptions_setTransformation(self.handle, value))
    }

    pub fn set_visibility(&self, value: Visibility) -> Result<()> {
        call!(dpiDeqOptions_setVisibility(self.handle, value.to_dpi()))
    }

    pub fn set_wait(&self, value: Duration) -> Result<()> {
        call!(dpiDeqOptions_setWait(
            self.handle,
            duration_to_secs(value, "deq wait")?
        ))
    }
}

impl DeqOptions {
    pub(crate) fn new(handle: *mut dpiDeqOptions) -> DeqOptions {
        DeqOptions { handle }
    }
}

impl Clone for DeqOptions {
    fn clone(&self) -> DeqOptions {
        unsafe { dpiDeqOptions_addRef(self.handle) };
        DeqOptions {
            handle: self.handle,
        }
    }
}

impl Drop for DeqOptions {
    fn drop(&mut self) {
        release_handle!(dpiDeqOptions_release(self.handle));
    }
}
