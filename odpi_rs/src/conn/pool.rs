use crate::conn::common_create_params::DpiCommonCreateParams;
use crate::conn::conn_create_params::DpiConnCreateParams;
use crate::conn::pool_create_params::DpiPoolCreateParams;
use crate::conn::{
    duration_to_millis, duration_to_secs, AccessToken, CommonCreateParams, Conn, ConnCreateParams,
    PoolCloseMode, PoolCreateParams, PoolGetMode,
};
use crate::context::Context;
use crate::maybe_async;
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::ffi::{c_char, c_int, c_void};
use std::fmt;
use std::pin::pin;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub type AccessTokenCallback = dyn Fn() -> Option<AccessToken> + Send + Sync + 'static;

pub(crate) struct AccessTokenCallbackContext {
    callback: Arc<AccessTokenCallback>,
    token: Mutex<AccessToken>,
}

impl AccessTokenCallbackContext {
    pub(crate) fn new(callback: &Arc<AccessTokenCallback>) -> Arc<AccessTokenCallbackContext> {
        Arc::new(AccessTokenCallbackContext {
            callback: callback.clone(),
            token: Mutex::new(AccessToken::new("", "")),
        })
    }

    pub(crate) unsafe extern "C" fn c_callback(
        context: *mut c_void,
        out_token: *mut dpiAccessToken,
    ) -> c_int {
        let self_ = &mut *(context as *mut Self);
        if let Some(token) = (self_.callback)() {
            unsafe {
                (*out_token).token = token.token.as_ptr() as *mut c_char;
                (*out_token).tokenLength = token.token.len() as u32;
                (*out_token).privateKey = token.private_key.as_ptr() as *mut c_char;
                (*out_token).privateKeyLength = token.private_key.len() as u32;
            }
            *self_.token.lock().unwrap() = token;
            DPI_SUCCESS as i32
        } else {
            DPI_FAILURE
        }
    }

    pub(crate) fn c_callback_context(self: &Arc<Self>) -> *mut c_void {
        Arc::as_ptr(self) as *mut c_void
    }
}

#[odpic_doc]
pub struct Pool {
    pub(crate) handle: *mut dpiPool,
    callback_context: Option<Arc<AccessTokenCallbackContext>>,
}

#[odpic_doc]
impl Pool {
    pub(crate) fn new(
        handle: *mut dpiPool,
        callback_context: Option<Arc<AccessTokenCallbackContext>>,
    ) -> Pool {
        Pool {
            handle,
            callback_context,
        }
    }

    #[maybe_async]
    pub async fn acquire_connection<U, P>(
        &self,
        username: U,
        password: P,
        create_params: Option<&mut ConnCreateParams>,
    ) -> Result<Conn>
    where
        U: AsRef<str>,
        P: AsRef<str>,
    {
        let mut dpi_create_params = pin!(DpiConnCreateParams::new(create_params)?);
        let conn = Conn::new(
            *get_value_blocking! {
                let handle = self.handle;
                let username_ptr = username.to_ptr();
                let username_len = username.try_to_len()?;
                let password_ptr = password.to_ptr();
                let password_len = password.try_to_len()?;
                let dpi_create_params_mut_ptr = dpi_create_params.as_mut_ptr();
                dpiPool_acquireConnection(
                    *handle,
                    *username_ptr,
                    *username_len,
                    *password_ptr,
                    *password_len,
                    *dpi_create_params_mut_ptr,
                )
            }
            .await?,
        );
        dpi_create_params.update_out_params()?;
        Ok(conn)
    }

    #[maybe_async]
    pub async fn close(&self, close_mode: PoolCloseMode) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            dpiPool_close(*handle, close_mode.to_dpi())
        }
        .await
    }

    #[maybe_async]
    pub async fn create<U, P, C>(
        username: U,
        password: P,
        connect_string: C,
        common_params: Option<&CommonCreateParams>,
        create_params: Option<&mut PoolCreateParams>,
    ) -> Result<Pool>
    where
        U: AsRef<str>,
        P: AsRef<str>,
        C: AsRef<str>,
    {
        let ctxt = Context::get()?;
        let mut common_params = pin!(DpiCommonCreateParams::new(&common_params)?);
        let mut dpi_create_params = pin!(DpiPoolCreateParams::new(create_params)?);
        let pool = Pool::new(
            *get_value_blocking! {
                let handle = ctxt.handle;
                let username_ptr = username.to_ptr();
                let username_len = username.try_to_len()?;
                let password_ptr = password.to_ptr();
                let password_len = password.try_to_len()?;
                let connect_string_ptr = connect_string.to_ptr();
                let connect_string_len = connect_string.try_to_len()?;
                let common_params_ptr = common_params.as_ptr();
                let dpi_create_params_mut_ptr = dpi_create_params.as_mut_ptr();
                dpiPool_create(
                    *handle,
                    *username_ptr,
                    *username_len,
                    *password_ptr,
                    *password_len,
                    *connect_string_ptr,
                    *connect_string_len,
                    *common_params_ptr,
                    *dpi_create_params_mut_ptr,
                )
            }
            .await?,
            dpi_create_params.callback_context(),
        );
        dpi_create_params.update_out_params()?;
        Ok(pool)
    }

    pub fn busy_count(&self) -> Result<u32> {
        get_value!(dpiPool_getBusyCount(self.handle))
    }

    pub fn get_mode(&self) -> Result<PoolGetMode> {
        get_value!(dpiPool_getGetMode(self.handle))?.try_to_rust()
    }

    pub fn max_lifetime_session(&self) -> Result<Duration> {
        Ok(Duration::from_secs(
            get_value!(dpiPool_getMaxLifetimeSession(self.handle))?.into(),
        ))
    }

    pub fn max_sessions_per_shard(&self) -> Result<u32> {
        get_value!(dpiPool_getMaxSessionsPerShard(self.handle))
    }

    pub fn open_count(&self) -> Result<u32> {
        get_value!(dpiPool_getOpenCount(self.handle))
    }

    pub fn soda_metadata_cache(&self) -> Result<bool> {
        Ok(get_value!(dpiPool_getSodaMetadataCache(self.handle))?.to_rust())
    }

    pub fn stmt_cache_size(&self) -> Result<u32> {
        get_value!(dpiPool_getStmtCacheSize(self.handle))
    }

    pub fn timeout(&self) -> Result<Duration> {
        Ok(Duration::from_secs(
            get_value!(dpiPool_getTimeout(self.handle))?.into(),
        ))
    }

    pub fn wait_timeout(&self) -> Result<Duration> {
        Ok(Duration::from_millis(
            get_value!(dpiPool_getWaitTimeout(self.handle))?.into(),
        ))
    }

    pub fn ping_interval(&self) -> Result<Option<Duration>> {
        let interval = get_value!(dpiPool_getPingInterval(self.handle))?;
        Ok(if interval < 0 {
            None
        } else {
            Some(Duration::from_secs(interval.try_into().unwrap()))
        })
    }

    #[maybe_async]
    pub async fn reconfigure(
        &self,
        min_sessions: u32,
        max_sessions: u32,
        session_increment: u32,
    ) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            dpiPool_reconfigure(
                *handle,
                min_sessions,
                max_sessions,
                session_increment
            )
        }
        .await
    }

    pub fn set_access_token(&self, params: &AccessToken) -> Result<()> {
        call!(dpiPool_setAccessToken(
            self.handle,
            &mut params.try_to_dpi()?
        ))
    }

    pub fn set_get_mode(&self, value: PoolGetMode) -> Result<()> {
        call!(dpiPool_setGetMode(self.handle, value.to_dpi()))
    }

    pub fn set_max_lifetime_session(&self, value: Duration) -> Result<()> {
        call!(dpiPool_setMaxLifetimeSession(
            self.handle,
            duration_to_secs(value, "max lifetime session")?
        ))
    }

    pub fn set_max_sessions_per_shard(&self, value: u32) -> Result<()> {
        call!(dpiPool_setMaxSessionsPerShard(self.handle, value))
    }

    pub fn set_soda_metadata_cache(&self, value: bool) -> Result<()> {
        call!(dpiPool_setSodaMetadataCache(self.handle, value.to_dpi()))
    }

    pub fn set_stmt_cache_size(&self, cache_size: u32) -> Result<()> {
        call!(dpiPool_setStmtCacheSize(self.handle, cache_size))
    }

    pub fn set_timeout(&self, value: Duration) -> Result<()> {
        call!(dpiPool_setTimeout(
            self.handle,
            duration_to_secs(value, "pool's timeout")?,
        ))
    }

    pub fn set_wait_timeout(&self, value: Duration) -> Result<()> {
        call!(dpiPool_setWaitTimeout(
            self.handle,
            duration_to_millis(value, "pool's wait timeout")?,
        ))
    }

    pub fn set_ping_interval(&self, value: Option<Duration>) -> Result<()> {
        call!(dpiPool_setPingInterval(
            self.handle,
            value.map_or(Ok(-1), |dur| duration_to_secs(dur, "pool ping interval"))?,
        ))
    }
}

impl Clone for Pool {
    fn clone(&self) -> Pool {
        unsafe { dpiPool_addRef(self.handle) };
        Pool {
            handle: self.handle,
            callback_context: self.callback_context.clone(),
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        release_handle!(dpiPool_release(self.handle));
    }
}

impl ToDpi<*mut dpiPool> for Option<Pool> {
    fn to_dpi(&self) -> *mut dpiPool {
        match self {
            Some(pool) => pool.handle,
            None => ptr::null_mut(),
        }
    }
}

impl fmt::Debug for Pool {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        // struct just writing "..." by fmt::Debug
        struct FmtEllipsis();
        impl fmt::Debug for FmtEllipsis {
            fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
                f.write_str("...")
            }
        }

        f.debug_struct("Pool")
            .field("hanlde", &self.handle)
            .field(
                "callback_context",
                &self.callback_context.as_ref().map(|_| FmtEllipsis()),
            )
            .finish_non_exhaustive()
    }
}
