//! Types for connection and connection pool

use crate::conn::common_create_params::DpiCommonCreateParams;
use crate::conn::conn_create_params::DpiConnCreateParams;
use crate::context::Context;
use crate::maybe_async;
use crate::stmt::{Stmt, Var};
use crate::subscr::subscr_create_params::DpiSubscrCreateParams;
use crate::subscr::{Subscr, SubscrCreateParams};
use crate::types::{Json, Lob, NativeType, ObjectType, OracleType, Vector, VectorInfo, Xid};
use crate::utils::*;
use crate::*;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::ffi::c_void;
use std::pin::pin;
use std::ptr;
use std::time::Duration;

mod access_token;
mod app_context;
mod common_create_params;
mod conn_create_params;
mod conn_info;
mod enums;
mod pool;
mod pool_create_params;
mod sharding_key_column;

pub use access_token::AccessToken;
pub use app_context::AppContext;
pub use common_create_params::CommonCreateParams;
pub use conn_create_params::ConnCreateParams;
pub use conn_info::ConnInfo;
pub use enums::*;
pub use pool::AccessTokenCallback;
pub use pool::Pool;
pub use pool_create_params::PoolCreateParams;
pub use sharding_key_column::ShardingKeyColumn;

#[derive(Debug)]
#[odpic_doc]
pub struct Conn {
    pub(crate) handle: *mut dpiConn,
}

unsafe impl Send for Conn {}
unsafe impl Sync for Conn {}

#[odpic_doc]
impl Conn {
    pub(crate) fn new(handle: *mut dpiConn) -> Conn {
        Conn { handle }
    }

    #[maybe_async]
    #[odpic_doc(name = "dpiConn_create")]
    pub async fn create<U, P, C>(
        username: U,
        password: P,
        connect_string: C,
        common_params: Option<&CommonCreateParams>,
        create_params: Option<&mut ConnCreateParams>,
    ) -> Result<Conn>
    where
        U: AsRef<str>,
        P: AsRef<str>,
        C: AsRef<str>,
    {
        let ctxt = Context::get()?;
        let mut common_params = pin!(DpiCommonCreateParams::new(&common_params)?);
        let mut dpi_create_params = pin!(DpiConnCreateParams::new(create_params)?);
        let conn = Conn::new(
            *get_value_blocking! {
                let handle = ctxt.handle;
                let username_ptr = username.to_ptr();
                let username_len = username.try_to_len()?;
                let password_ptr = password.to_ptr();
                let password_len = password.try_to_len()?;
                let connect_string_ptr = connect_string.to_ptr();
                let connect_string_len = connect_string.try_to_len()?;
                let common_params_ptr = common_params.as_ptr();
                let dpi_create_params_ptr = dpi_create_params.as_mut_ptr();
                dpiConn_create(
                    *handle,
                    *username_ptr,
                    *username_len,
                    *password_ptr,
                    *password_len,
                    *connect_string_ptr,
                    *connect_string_len,
                    *common_params_ptr,
                    *dpi_create_params_ptr,
            )}
            .await?,
        );
        dpi_create_params.update_out_params()?;
        Ok(conn)
    }

    #[maybe_async]
    pub async fn break_execution(&self) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            dpiConn_breakExecution(*handle)
        }
        .await
    }

    #[maybe_async]
    pub async fn change_password<U, O, N>(
        &self,
        user_name: U,
        old_password: O,
        new_password: N,
    ) -> Result<()>
    where
        U: AsRef<str>,
        O: AsRef<str>,
        N: AsRef<str>,
    {
        call_blocking! {
            let handle = self.handle;
            let user_name_ptr =  user_name.to_ptr();
            let user_name_len = user_name.try_to_len()?;
            let old_password_ptr = old_password.to_ptr();
            let old_password_len = old_password.try_to_len()?;
            let new_password_ptr = new_password.to_ptr();
            let new_password_len = new_password.try_to_len()?;
            dpiConn_changePassword(
                *handle,
                *user_name_ptr,
                *user_name_len,
                *old_password_ptr,
                *old_password_len,
                *new_password_ptr,
                *new_password_len,
            )
        }
        .await
    }

    #[maybe_async]
    pub async fn close<T>(&self, mode: ConnCloseMode, tag: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        call_blocking! {
            let handle = self.handle;
            let tag_ptr = tag.to_ptr();
            let tag_len = tag.try_to_len()?;
            dpiConn_close(
                *handle,
                mode.to_dpi(),
                *tag_ptr,
                *tag_len,
            )
        }
        .await
    }

    #[maybe_async]
    pub async fn commit(&self) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            dpiConn_commit(*handle)
        }
        .await
    }

    pub fn call_timeout(&self) -> Result<Duration> {
        Ok(Duration::from_millis(
            get_value!(dpiConn_getCallTimeout(self.handle))?.into(),
        ))
    }

    pub fn current_schema(&self) -> Result<String> {
        get_string_value!(dpiConn_getCurrentSchema(self.handle))
    }

    pub fn db_domain(&self) -> Result<String> {
        get_string_value!(dpiConn_getDbDomain(self.handle))
    }

    pub fn db_name(&self) -> Result<String> {
        get_string_value!(dpiConn_getDbName(self.handle))
    }

    pub fn edition(&self) -> Result<String> {
        get_string_value!(dpiConn_getEdition(self.handle))
    }

    pub fn external_name(&self) -> Result<String> {
        get_string_value!(dpiConn_getExternalName(self.handle))
    }

    pub fn handle(&self) -> Result<*mut c_void> {
        get_value!(dpiConn_getHandle(self.handle))
    }

    pub fn info(&self) -> Result<ConnInfo> {
        get_value!(dpiConn_getInfo(self.handle))?.try_to_rust()
    }

    pub fn instance_name(&self) -> Result<String> {
        get_string_value!(dpiConn_getInstanceName(self.handle))
    }

    pub fn internal_name(&self) -> Result<String> {
        get_string_value!(dpiConn_getInternalName(self.handle))
    }

    pub fn is_healthy(&self) -> Result<bool> {
        Ok(get_value!(dpiConn_getIsHealthy(self.handle))?.to_rust())
    }

    #[odpic_doc(name = "dpiConn_getLTXID")]
    pub fn ltxid(&self) -> Result<Vec<u8>> {
        let mut ptr = ptr::null();
        let mut len = 0;
        call!(dpiConn_getLTXID(self.handle, &mut ptr, &mut len))?;
        (ptr, len).try_to_rust()
    }

    pub fn max_open_cursors(&self) -> Result<u32> {
        get_value!(dpiConn_getMaxOpenCursors(self.handle))
    }

    #[maybe_async]
    pub async fn object_type<T>(&self, name: T) -> Result<ObjectType>
    where
        T: AsRef<str>,
    {
        Ok(ObjectType::new(
            *get_value_blocking! {
                let handle = self.handle;
                let name_ptr = name.to_ptr();
                let name_len = name.try_to_len()?;
                dpiConn_getObjectType(
                    *handle,
                    *name_ptr,
                    *name_len,
                )
            }
            .await?,
        ))
    }

    pub unsafe fn oci_attr(
        &self,
        handle_type: u32,
        attribute: u32,
    ) -> Result<(*const c_char, u32)> {
        let (buffer, len) = get_2values!(dpiConn_getOciAttr(self.handle, handle_type, attribute))?;
        Ok((buffer.asRaw as *const c_char, len))
    }

    #[maybe_async]
    pub async fn server_version(&self) -> Result<(String, VersionInfo)> {
        let (ptr, len, ver) = get_3values_blocking! {
            let handle = self.handle;
            dpiConn_getServerVersion(*handle)
        }
        .await?;
        Ok(((*ptr, *len).try_to_rust()?, (*ver).into()))
    }

    pub fn service_name(&self) -> Result<String> {
        get_string_value!(dpiConn_getServiceName(self.handle))
    }

    #[maybe_async]
    pub async fn soda_db(&self) -> Result<soda::Db> {
        Ok(soda::Db::new(
            *get_value_blocking! {
                let handle = self.handle;
                dpiConn_getSodaDb(*handle)
            }
            .await?,
        ))
    }

    pub fn stmt_cache_size(&self) -> Result<u32> {
        get_value!(dpiConn_getStmtCacheSize(self.handle))
    }

    pub fn transaction_in_progress(&self) -> Result<bool> {
        Ok(get_value!(dpiConn_getTransactionInProgress(self.handle))?.to_rust())
    }

    pub fn new_deq_options(&self) -> Result<aq::DeqOptions> {
        Ok(aq::DeqOptions::new(get_value!(dpiConn_newDeqOptions(
            self.handle
        ))?))
    }

    pub fn new_enq_options(&self) -> Result<aq::EnqOptions> {
        Ok(aq::EnqOptions::new(get_value!(dpiConn_newEnqOptions(
            self.handle
        ))?))
    }

    pub fn new_json(&self) -> Result<Json> {
        Ok(Json::new(get_value!(dpiConn_newJson(self.handle))?))
    }

    pub fn new_json_queue<T>(&self, name: T) -> Result<aq::Queue>
    where
        T: AsRef<str>,
    {
        Ok(aq::Queue::new(get_value!(dpiConn_newJsonQueue(
            self.handle,
            name.to_ptr(),
            name.try_to_len()?
        ))?))
    }

    pub fn new_msg_props(&self) -> Result<aq::MsgProps> {
        Ok(aq::MsgProps::new(get_value!(dpiConn_newMsgProps(
            self.handle
        ))?))
    }

    pub fn new_queue<T>(&self, name: T, palyload_type: Option<&ObjectType>) -> Result<aq::Queue>
    where
        T: AsRef<str>,
    {
        Ok(aq::Queue::new(get_value!(dpiConn_newQueue(
            self.handle,
            name.to_ptr(),
            name.try_to_len()?,
            palyload_type
                .map(|payload| payload.handle)
                .unwrap_or(ptr::null_mut())
        ))?))
    }

    #[maybe_async]
    pub async fn new_temp_lob(&self, lob_type: OracleType) -> Result<Lob> {
        Ok(Lob::new(
            *get_value_blocking! {
                let handle = self.handle;
                dpiConn_newTempLob(
                    *handle,
                    lob_type.to_dpi()
                )
            }
            .await?,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_var(
        &self,
        oracle_type: OracleType,
        native_type: NativeType,
        max_array_size: u32,
        size: u32,
        size_is_byte: bool,
        is_array: bool,
        obj_type: Option<&ObjectType>,
    ) -> Result<Var> {
        let (handle, data) = get_2values!(dpiConn_newVar(
            self.handle,
            oracle_type.to_dpi(),
            native_type.to_dpi(),
            max_array_size,
            size,
            size_is_byte.to_dpi(),
            is_array.to_dpi(),
            obj_type.to_dpi(),
        ))?;
        Ok(Var::new(handle, native_type, data))
    }

    pub fn new_vector(&self, info: Option<&VectorInfo>) -> Result<Vector> {
        let mut local_info;
        let info_ptr = if let Some(info) = info {
            local_info = info.to_dpi();
            &mut local_info
        } else {
            ptr::null_mut()
        };
        Ok(Vector::new(get_value!(dpiConn_newVector(
            self.handle,
            info_ptr
        ))?))
    }

    #[maybe_async]
    pub async fn ping(&self) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            dpiConn_ping(*handle)
        }
        .await
    }

    pub fn prepare_stmt<S, T>(&self, scrollable: bool, sql: S, tag: T) -> Result<Stmt>
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Ok(Stmt::new(get_value!(dpiConn_prepareStmt(
            self.handle,
            scrollable.to_dpi(),
            sql.to_ptr(),
            sql.try_to_len()?,
            tag.to_ptr(),
            tag.try_to_len()?,
        ))?))
    }

    #[maybe_async]
    pub async fn rollback(&self) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            dpiConn_rollback(*handle)
        }
        .await
    }

    pub fn set_action<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        call!(dpiConn_setAction(
            self.handle,
            value.to_ptr(),
            value.try_to_len()?
        ))
    }

    pub fn set_call_timeout(&self, value: Duration) -> Result<()> {
        call!(dpiConn_setCallTimeout(
            self.handle,
            duration_to_millis(value, "call timeout")?
        ))
    }

    pub fn set_client_identifier<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        call!(dpiConn_setClientIdentifier(
            self.handle,
            value.to_ptr(),
            value.try_to_len()?
        ))
    }

    pub fn set_client_info<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        call!(dpiConn_setClientInfo(
            self.handle,
            value.to_ptr(),
            value.try_to_len()?
        ))
    }

    pub fn set_current_schema<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        call!(dpiConn_setCurrentSchema(
            self.handle,
            value.to_ptr(),
            value.try_to_len()?
        ))
    }

    pub fn set_db_op<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        call!(dpiConn_setDbOp(
            self.handle,
            value.to_ptr(),
            value.try_to_len()?
        ))
    }

    pub fn set_econtext_id<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        call!(dpiConn_setEcontextId(
            self.handle,
            value.to_ptr(),
            value.try_to_len()?
        ))
    }

    pub fn set_external_name<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        call!(dpiConn_setExternalName(
            self.handle,
            value.to_ptr(),
            value.try_to_len()?
        ))
    }

    pub fn set_internal_name<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        call!(dpiConn_setInternalName(
            self.handle,
            value.to_ptr(),
            value.try_to_len()?
        ))
    }

    pub fn set_module<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        call!(dpiConn_setModule(
            self.handle,
            value.to_ptr(),
            value.try_to_len()?
        ))
    }

    pub unsafe fn set_oci_attr(
        &self,
        handle_type: u32,
        attribute: u32,
        value: *mut c_void,
        value_length: u32,
    ) -> Result<()> {
        call!(dpiConn_setOciAttr(
            self.handle,
            handle_type,
            attribute,
            value,
            value_length
        ))
    }

    pub fn set_stmt_cache_size<T>(&self, value: T) -> Result<()>
    where
        T: Into<u32>,
    {
        call!(dpiConn_setStmtCacheSize(self.handle, value.into()))
    }

    #[maybe_async]
    pub async fn shutdown_database(&self, mode: ShutdownMode) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            dpiConn_shutdownDatabase(*handle, mode.to_dpi())
        }
        .await
    }

    #[maybe_async]
    pub async fn startup_database(&self, mode: StartupMode) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            dpiConn_startupDatabase(*handle, mode.to_dpi())
        }
        .await
    }

    #[maybe_async]
    pub async fn startup_database_with_pfile<T>(&self, pfile: T, mode: StartupMode) -> Result<()>
    where
        T: AsRef<str>,
    {
        call_blocking! {
            let handle = self.handle;
            let pfile_ptr = pfile.to_ptr();
            let pfile_len = pfile.try_to_len()?;
            dpiConn_startupDatabaseWithPfile(
                *handle,
                *pfile_ptr,
                *pfile_len,
                mode.to_dpi()
            )
        }
        .await
    }

    #[maybe_async]
    pub async fn subscribe(&self, params: &mut SubscrCreateParams) -> Result<Subscr> {
        let mut params = pin!(DpiSubscrCreateParams::new(params)?);
        let subscr = Subscr::new(
            *get_value_blocking! {
                let handle = self.handle;
                let params_ptr = params.as_mut_ptr();
                dpiConn_subscribe(*handle, *params_ptr)
            }
            .await?,
            params.callback_context(),
        );
        params.update_out_params();
        Ok(subscr)
    }

    #[maybe_async]
    pub async fn tpc_begin(
        &self,
        xid: &Xid<'_>,
        transaction_timeout: u32,
        flags: TpcBeginFlags,
    ) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            let mut xid = xid.to_dpi();
            dpiConn_tpcBegin(
                *handle,
                &mut *xid,
                transaction_timeout,
                flags.to_dpi()
            )
        }
        .await
    }

    #[maybe_async]
    pub async fn tpc_commit(&self, xid: &Xid<'_>, one_phase: bool) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            let mut xid = xid.to_dpi();
            dpiConn_tpcCommit(*handle, &mut *xid, one_phase.to_dpi())
        }
        .await
    }

    #[maybe_async]
    pub async fn tpc_end(&self, xid: &Xid<'_>, flags: TpcEndFlags) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            let mut xid = xid.to_dpi();
            dpiConn_tpcEnd(*handle, &mut *xid, flags.to_dpi())
        }
        .await
    }

    #[maybe_async]
    pub async fn tpc_forget(&self, xid: &Xid<'_>) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            let mut xid = xid.to_dpi();
            dpiConn_tpcForget(*handle, &mut *xid)
        }
        .await
    }

    #[maybe_async]
    pub async fn tpc_prepare(&self, xid: &Xid<'_>) -> Result<bool> {
        Ok(*get_value_blocking! {
            let handle = self.handle;
            let mut xid = xid.to_dpi();
            dpiConn_tpcPrepare(*handle, &mut *xid)
        }
        .await?
            != 0)
    }

    #[maybe_async]
    pub async fn tpc_rollback(&self, xid: &Xid<'_>) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            let mut xid = xid.to_dpi();
            dpiConn_tpcRollback(*handle, &mut *xid)
        }
        .await
    }

    #[maybe_async]
    pub async fn unsubscribe(&self, subscr: &subscr::Subscr) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            let subscr_handle = subscr.handle;
            dpiConn_unsubscribe(*handle, *subscr_handle)
        }
        .await
    }
}

impl Clone for Conn {
    fn clone(&self) -> Conn {
        unsafe { dpiConn_addRef(self.handle) };
        Conn {
            handle: self.handle,
        }
    }
}

impl Drop for Conn {
    fn drop(&mut self) {
        release_handle!(dpiConn_release(self.handle));
    }
}
