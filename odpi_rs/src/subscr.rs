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
//! Subscriptions to events such as continuous query notification and object change notification

use crate::stmt::Stmt;
use crate::utils::*;
use crate::*;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::ffi::c_void;
use std::fmt;
use std::sync::Arc;

mod enums;
mod message;
pub(crate) mod subscr_create_params;

pub use enums::*;
pub use message::*;
pub use subscr_create_params::SubscrCreateParams;

pub type SubscrCallback = dyn Fn(Result<Message>) + Send + Sync + 'static;

pub(crate) struct SubscrCallbackContext {
    callback: Arc<SubscrCallback>,
}

impl SubscrCallbackContext {
    pub(crate) fn new(callback: &Arc<SubscrCallback>) -> Arc<SubscrCallbackContext> {
        Arc::new(SubscrCallbackContext {
            callback: callback.clone(),
        })
    }

    pub(crate) unsafe extern "C" fn c_callback(
        context: *mut c_void,
        message: *mut dpiSubscrMessage,
    ) {
        let self_ = &mut *(context as *mut Self);
        let message = &*message;
        let message = if message.errorInfo.is_null() {
            Message::new(message)
        } else {
            Err(OdpiError::from_dpi(&*message.errorInfo).into())
        };
        (self_.callback)(message);
    }

    pub(crate) fn c_callback_context(self: &Arc<Self>) -> *mut c_void {
        Arc::as_ptr(self) as *mut c_void
    }
}

#[odpic_doc]
pub struct Subscr {
    pub(crate) handle: *mut dpiSubscr,
    callback_context: Option<Arc<SubscrCallbackContext>>,
}

#[odpic_doc]
impl Subscr {
    pub(crate) fn new(
        handle: *mut dpiSubscr,
        callback_context: Option<Arc<SubscrCallbackContext>>,
    ) -> Subscr {
        Subscr {
            handle,
            callback_context,
        }
    }

    pub fn prepare_stmt<T>(&self, sql: T) -> Result<Stmt>
    where
        T: AsRef<str>,
    {
        Ok(Stmt::new(get_value!(dpiSubscr_prepareStmt(
            self.handle,
            sql.to_ptr(),
            sql.try_to_len()?
        ))?))
    }
}

impl Clone for Subscr {
    fn clone(&self) -> Subscr {
        unsafe { dpiSubscr_addRef(self.handle) };
        Subscr {
            handle: self.handle,
            callback_context: self.callback_context.clone(),
        }
    }
}

impl Drop for Subscr {
    fn drop(&mut self) {
        release_handle!(dpiSubscr_release(self.handle));
    }
}

impl fmt::Debug for Subscr {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        f.debug_struct("Subscr")
            .field("handle", &self.handle)
            .field(
                "callback_context",
                &self.callback_context.as_ref().map(|_| FmtEllipsis()),
            )
            .finish()
    }
}
