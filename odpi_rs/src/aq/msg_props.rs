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
use crate::aq::{MessageDeliveryMode, MessageState, MsgRecipient};
use crate::types::{Json, Object, Timestamp};
use crate::utils::*;
use crate::Error;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::time::Duration;

#[derive(Debug)]
#[odpic_doc]
pub struct MsgProps {
    pub(crate) handle: *mut dpiMsgProps,
}

#[odpic_doc]
impl MsgProps {
    pub(crate) fn new(handle: *mut dpiMsgProps) -> MsgProps {
        MsgProps { handle }
    }

    pub fn num_attempts(&self) -> Result<i32> {
        Ok(get_value!(dpiMsgProps_getNumAttempts(self.handle))?)
    }

    pub fn correlation(&self) -> Result<String> {
        get_string_value!(dpiMsgProps_getCorrelation(self.handle))
    }

    pub fn delay(&self) -> Result<Duration> {
        let value = get_value!(dpiMsgProps_getDelay(self.handle))?;
        if value > 0 {
            Ok(Duration::from_secs(value.try_into().unwrap()))
        } else {
            Err(Error::other(format!("MsgProps delay is {}", value)))
        }
    }

    pub fn delivery_mode(&self) -> Result<MessageDeliveryMode> {
        get_value!(dpiMsgProps_getDeliveryMode(self.handle))?.try_to_rust()
    }

    pub fn enq_time(&self) -> Result<Timestamp> {
        Ok(get_value!(dpiMsgProps_getEnqTime(self.handle))?.into())
    }

    pub fn exception_q(&self) -> Result<String> {
        get_string_value!(dpiMsgProps_getExceptionQ(self.handle))
    }

    pub fn expiration(&self) -> Result<Option<Duration>> {
        let value = get_value!(dpiMsgProps_getExpiration(self.handle))?;
        if value >= 0 {
            Ok(Some(Duration::from_secs(value.try_into().unwrap())))
        } else if value == -1 {
            Ok(None)
        } else {
            Err(Error::other(format!("MsgProps expiration is {}", value)))
        }
    }

    pub fn msg_id(&self) -> Result<String> {
        get_string_value!(dpiMsgProps_getMsgId(self.handle))
    }

    pub fn original_msg_id(&self) -> Result<String> {
        get_string_value!(dpiMsgProps_getOriginalMsgId(self.handle))
    }

    pub fn payload(&self) -> Result<(Option<Object>, Option<Vec<u8>>)> {
        let (obj, value, value_len) = get_3values!(dpiMsgProps_getPayload(self.handle))?;
        let obj = if obj.is_null() {
            None
        } else {
            Some(Object::with_add_ref(obj))
        };
        let bytes = if value.is_null() {
            None
        } else {
            Some((value, value_len).try_to_rust()?)
        };
        Ok((obj, bytes))
    }

    pub fn payload_json(&self) -> Result<Json> {
        Ok(Json::with_add_ref(get_value!(dpiMsgProps_getPayloadJson(
            self.handle
        ))?))
    }

    pub fn priority(&self) -> Result<i32> {
        get_value!(dpiMsgProps_getPriority(self.handle))
    }

    pub fn state(&self) -> Result<MessageState> {
        get_value!(dpiMsgProps_getState(self.handle))?.try_to_rust()
    }

    pub fn set_correlation<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        set_str_value!(dpiMsgProps_setCorrelation(self.handle, value))
    }

    pub fn set_delay(&self, value: Duration) -> Result<()> {
        call!(dpiMsgProps_setDelay(
            self.handle,
            duration_to_secs(value, "MsgProps delay")?
        ))
    }

    pub fn set_exception_q<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        set_str_value!(dpiMsgProps_setExceptionQ(self.handle, value))
    }

    pub fn set_expiration(&self, value: Option<Duration>) -> Result<()> {
        let value = if let Some(value) = value {
            duration_to_secs(value, "MsgProps expiration")?
        } else {
            -1
        };
        call!(dpiMsgProps_setExpiration(self.handle, value))
    }

    pub fn set_original_msg_id<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        set_str_value!(dpiMsgProps_setOriginalMsgId(self.handle, value))
    }

    pub fn set_payload_bytes<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<[u8]>,
    {
        let value = value.as_ref();
        call!(dpiMsgProps_setPayloadBytes(
            self.handle,
            value.as_ptr().to_dpi(),
            value.len().try_into()?
        ))
    }

    pub fn set_payload_json(&self, value: &Json) -> Result<()> {
        call!(dpiMsgProps_setPayloadJson(self.handle, value.handle))
    }

    pub fn set_payload_object(&self, value: &Object) -> Result<()> {
        call!(dpiMsgProps_setPayloadObject(self.handle, value.handle))
    }

    pub fn set_priority(&self, value: i32) -> Result<()> {
        call!(dpiMsgProps_setPriority(self.handle, value))
    }

    pub fn set_recipients<'a, T>(&self, values: T) -> Result<()>
    where
        T: IntoIterator<Item = &'a MsgRecipient<'a>>,
    {
        let mut values = values
            .into_iter()
            .map(<MsgRecipient as TryToDpi<dpiMsgRecipient>>::try_to_dpi)
            .collect::<Result<Vec<_>>>()?;
        call!(dpiMsgProps_setRecipients(
            self.handle,
            values.as_mut_ptr(),
            values.len().try_into()?
        ))
    }
}

impl Clone for MsgProps {
    fn clone(&self) -> MsgProps {
        unsafe { dpiMsgProps_addRef(self.handle) };
        MsgProps {
            handle: self.handle,
        }
    }
}

impl Drop for MsgProps {
    fn drop(&mut self) {
        release_handle!(dpiMsgProps_release(self.handle));
    }
}
