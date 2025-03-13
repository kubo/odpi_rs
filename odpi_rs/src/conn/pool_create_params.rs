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
use crate::conn::{
    pool::AccessTokenCallbackContext, AccessToken, AccessTokenCallback, PoolGetMode,
};
use crate::context::Context;
use crate::utils::*;
use crate::{AssertSend, AssertSync, Result};
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::fmt;
use std::ptr;
use std::sync::Arc;
use std::time::Duration;

#[non_exhaustive]
#[odpic_doc]
pub struct PoolCreateParams {
    pub min_sessions: u32,
    pub max_sessions: u32,
    pub session_increment: u32,
    pub ping_interval: Option<Duration>,
    pub ping_timeout: Duration,
    pub homogeneous: bool,
    pub external_auth: bool,
    pub get_mode: PoolGetMode,
    pub out_pool_name: String,
    pub timeout: Duration,
    pub wait_timeout: Duration,
    pub max_lifetime_session: u32,
    pub plsql_fixup_callback: String,
    pub max_sessions_per_shard: u32,
    pub access_token_callback: Option<Arc<AccessTokenCallback>>,
}

impl PoolCreateParams {
    /// Create `PoolCreateParams`
    ///
    /// # Note
    /// This intanally calls [`Context::get()`].
    ///
    /// If you need to call [`Context::init()`], it must be called before this.
    pub fn new() -> Result<PoolCreateParams> {
        let ctxt = Context::get()?;
        let params = get_value!(dpiContext_initPoolCreateParams(ctxt.handle))?;
        Ok(PoolCreateParams {
            min_sessions: params.minSessions,
            max_sessions: params.maxSessions,
            session_increment: params.sessionIncrement,
            ping_interval: if params.pingInterval > 0 {
                Some(Duration::from_secs(params.pingInterval as u64))
            } else {
                None
            },
            ping_timeout: Duration::from_millis(params.pingTimeout.try_into()?),
            homogeneous: params.homogeneous.to_rust(),
            external_auth: params.externalAuth.to_rust(),
            get_mode: params.getMode.try_to_rust()?,
            out_pool_name: "".into(),
            timeout: Duration::from_secs(params.timeout.into()),
            wait_timeout: Duration::from_millis(params.waitTimeout.into()),
            max_lifetime_session: params.maxLifetimeSession,
            plsql_fixup_callback: "".into(),
            max_sessions_per_shard: params.maxSessionsPerShard,
            access_token_callback: None,
        })
    }

    /// Sets `value` to [`field@PoolCreateParams::min_sessions`] field.
    pub fn min_sessions<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<u32>,
    {
        self.min_sessions = value.into();
        self
    }

    /// Sets `value` to [`field@PoolCreateParams::max_sessions`] field.
    pub fn max_sessions<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<u32>,
    {
        self.max_sessions = value.into();
        self
    }

    /// Sets `value` to [`field@PoolCreateParams::session_increment`] field.
    pub fn session_increment<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<u32>,
    {
        self.session_increment = value.into();
        self
    }

    /// Sets `value` to [`field@PoolCreateParams::ping_interval`] field.
    pub fn ping_interval(&mut self, value: Duration) -> &mut Self {
        self.ping_interval = Some(value);
        self
    }

    /// Sets `value` to [`field@PoolCreateParams::ping_timeout`] field.
    pub fn ping_timeout(&mut self, value: Duration) -> &mut Self {
        self.ping_timeout = value;
        self
    }

    /// Sets `value` to [`field@PoolCreateParams::homogeneous`] field.
    pub fn homogeneous<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<bool>,
    {
        self.homogeneous = value.into();
        self
    }

    /// Sets `value` to [`field@PoolCreateParams::external_auth`] field.
    pub fn external_auth<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<bool>,
    {
        self.external_auth = value.into();
        self
    }

    /// Sets `value` to [`field@PoolCreateParams::get_mode`] field.
    pub fn get_mode(&mut self, value: PoolGetMode) -> &mut Self {
        self.get_mode = value;
        self
    }

    /// Sets `value` to [`field@PoolCreateParams::timeout`] field.
    pub fn timeout(&mut self, value: Duration) -> &mut Self {
        self.timeout = value;
        self
    }

    /// Sets `value` to [`field@PoolCreateParams::wait_timeout`] field.
    pub fn wait_timeout(&mut self, value: Duration) -> &mut Self {
        self.wait_timeout = value;
        self
    }

    /// Sets `value` to [`field@PoolCreateParams::max_lifetime_session`] field.
    pub fn max_lifetime_session<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<u32>,
    {
        self.max_lifetime_session = value.into();
        self
    }

    /// Sets `value` to [`field@PoolCreateParams::plsql_fixup_callback`] field.
    pub fn plsql_fixup_callback<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.plsql_fixup_callback = value.into();
        self
    }

    /// Sets `value` to [`field@PoolCreateParams::max_sessions_per_shard`] field.
    pub fn max_sessions_per_shard<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<u32>,
    {
        self.max_sessions_per_shard = value.into();
        self
    }

    /// Sets `value` to [`field@PoolCreateParams::access_token_callback`] field.
    pub fn access_token_callback<T>(&mut self, value: T) -> &mut Self
    where
        T: Fn() -> Option<AccessToken> + Send + Sync + 'static,
    {
        self.access_token_callback = Some(Arc::new(value));
        self
    }
}

impl AssertSend for PoolCreateParams {}
impl AssertSync for PoolCreateParams {}

impl fmt::Debug for PoolCreateParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        f.debug_struct("PoolCreateParams")
            .field("min_sessions", &self.min_sessions)
            .field("max_sessions", &self.max_sessions)
            .field("session_increment", &self.session_increment)
            .field("ping_interval", &self.ping_interval)
            .field("ping_timeout", &self.ping_timeout)
            .field("homogeneous", &self.homogeneous)
            .field("external_auth", &self.external_auth)
            .field("get_mode", &self.get_mode)
            .field("out_pool_name", &self.out_pool_name)
            .field("timeout", &self.timeout)
            .field("wait_timeout", &self.wait_timeout)
            .field("max_lifetime_session", &self.max_lifetime_session)
            .field("plsql_fixup_callback", &self.plsql_fixup_callback)
            .field("max_sessions_per_shard", &self.max_sessions_per_shard)
            .field(
                "access_token_callback",
                &self.access_token_callback.as_ref().map(|_| FmtEllipsis()),
            )
            .finish_non_exhaustive()
    }
}

pub(crate) struct DpiPoolCreateParams<'a> {
    dpi_params: dpiPoolCreateParams,
    params: Option<&'a mut PoolCreateParams>,
    pub(crate) callback_context: Option<Arc<AccessTokenCallbackContext>>,
}

impl<'a> DpiPoolCreateParams<'a> {
    pub(crate) fn new(
        opt_params: Option<&'a mut PoolCreateParams>,
    ) -> Result<DpiPoolCreateParams<'a>> {
        if let Some(params) = &opt_params {
            let callback_context = params
                .access_token_callback
                .as_ref()
                .map(AccessTokenCallbackContext::new);
            Ok(DpiPoolCreateParams {
                dpi_params: dpiPoolCreateParams {
                    minSessions: params.min_sessions,
                    maxSessions: params.max_sessions,
                    sessionIncrement: params.session_increment,
                    pingInterval: params.ping_interval.map_or(Ok(-1), |duration| {
                        duration_to_secs(duration, "pool ping interval")
                    })?,
                    pingTimeout: duration_to_millis(params.ping_timeout, "pool's ping timeout")?,
                    homogeneous: params.homogeneous.to_dpi(),
                    externalAuth: params.external_auth.to_dpi(),
                    getMode: params.get_mode.to_dpi(),
                    outPoolName: ptr::null_mut(),
                    outPoolNameLength: 0,
                    timeout: duration_to_secs(params.timeout, "pool's timeout")?,
                    waitTimeout: duration_to_millis(params.wait_timeout, "pool's wait timeout")?,
                    maxLifetimeSession: params.max_lifetime_session,
                    plsqlFixupCallback: params.plsql_fixup_callback.to_ptr(),
                    plsqlFixupCallbackLength: params.plsql_fixup_callback.try_to_len()?,
                    maxSessionsPerShard: params.max_sessions_per_shard,
                    accessTokenCallback: if callback_context.is_some() {
                        Some(AccessTokenCallbackContext::c_callback)
                    } else {
                        None
                    },
                    accessTokenCallbackContext: if let Some(ctxt) = &callback_context {
                        ctxt.c_callback_context()
                    } else {
                        ptr::null_mut()
                    },
                },
                params: opt_params,
                callback_context,
            })
        } else {
            let ctxt = Context::get()?;
            let dpi_params = get_value!(dpiContext_initPoolCreateParams(ctxt.handle))?;
            Ok(DpiPoolCreateParams {
                dpi_params,
                params: opt_params,
                callback_context: None,
            })
        }
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut dpiPoolCreateParams {
        &mut self.dpi_params
    }

    pub(crate) fn callback_context(&mut self) -> Option<Arc<AccessTokenCallbackContext>> {
        self.callback_context.take()
    }

    pub(crate) fn update_out_params(&mut self) -> Result<()> {
        if let Some(params) = &mut self.params {
            let prms = &self.dpi_params;
            params.out_pool_name = (prms.outPoolName, prms.outPoolNameLength).try_to_rust()?;
        }
        Ok(())
    }
}
