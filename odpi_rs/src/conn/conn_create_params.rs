use crate::conn::{AppContext, AuthMode, Pool, Purity, ShardingKeyColumn};
use crate::context::Context;
use crate::utils::*;
use crate::*;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::ffi::c_void;
use std::ptr;

#[derive(Debug, Clone)]
#[non_exhaustive]
#[odpic_doc]
pub struct ConnCreateParams {
    pub auth_mode: AuthMode,
    pub connection_class: String,
    pub purity: Purity,
    pub new_password: String,
    pub app_context: Vec<AppContext>,
    pub external_auth: bool,
    pub external_handle: *mut c_void,
    pub pool: Option<Pool>,
    pub tag: String,
    pub match_any_tag: bool,
    pub out_tag: Option<String>,
    pub sharding_key_columns: Vec<ShardingKeyColumn>,
    pub super_sharding_key_columns: Vec<ShardingKeyColumn>,
    pub out_new_session: bool,
}

impl ConnCreateParams {
    /// Create `ConnCreateParams`
    ///
    /// # Note
    /// This intanally calls [`Context::get()`].
    ///
    /// If you need to call [`Context::init()`], it must be called before this.
    pub fn new() -> Result<ConnCreateParams> {
        let ctxt = Context::get()?;
        let params = get_value!(dpiContext_initConnCreateParams(ctxt.handle))?;
        Ok(ConnCreateParams {
            auth_mode: AuthMode::from_bits_truncate(params.authMode),
            connection_class: (params.connectionClass, params.connectionClassLength)
                .try_to_rust()?,
            purity: params.purity.try_to_rust()?,
            new_password: (params.newPassword, params.newPasswordLength).try_to_rust()?,
            app_context: Vec::new(),
            external_auth: params.externalAuth.to_rust(),
            external_handle: params.externalHandle,
            pool: None,
            tag: (params.tag, params.tagLength).try_to_rust()?,
            match_any_tag: params.matchAnyTag.to_rust(),
            out_tag: None,
            sharding_key_columns: Vec::new(),
            super_sharding_key_columns: Vec::new(),
            out_new_session: false,
        })
    }

    /// Sets `value` to [`field@ConnCreateParams::auth_mode`] field.
    pub fn auth_mode(&mut self, value: AuthMode) -> &mut Self {
        self.auth_mode = value;
        self
    }

    /// Sets `value` to [`field@ConnCreateParams::connection_class`] field.
    pub fn connection_class<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.connection_class = value.into();
        self
    }

    /// Sets `value` to [`field@ConnCreateParams::purity`] field.
    pub fn purity(&mut self, value: Purity) -> &mut Self {
        self.purity = value;
        self
    }

    /// Sets `value` to [`field@ConnCreateParams::new_password`] field.
    pub fn new_password<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.new_password = value.into();
        self
    }

    /// Add `value` to [`field@ConnCreateParams::app_context`] field.
    pub fn app_context(&mut self, value: AppContext) -> &mut Self {
        self.app_context.push(value);
        self
    }

    /// Add multiple values to [`field@ConnCreateParams::app_context`] field.
    pub fn app_contexts<T>(&mut self, values: T) -> &mut Self
    where
        T: IntoIterator<Item = AppContext>,
    {
        self.app_context.extend(values);
        self
    }

    /// Sets `value` to [`field@ConnCreateParams::external_auth`] field.
    pub fn external_auth(&mut self, value: bool) -> &mut Self {
        self.external_auth = value;
        self
    }

    /// Sets `value` to [`field@ConnCreateParams::external_handle`] field.
    pub fn external_handle(&mut self, value: *mut c_void) -> &mut Self {
        self.external_handle = value;
        self
    }

    /// Sets `value` to [`field@ConnCreateParams::pool`] field.
    pub fn pool(&mut self, value: Pool) -> &mut Self {
        self.pool = Some(value);
        self
    }

    /// Sets `value` to [`field@ConnCreateParams::tag`] field.
    pub fn tag<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.tag = value.into();
        self
    }

    /// Sets `value` to [`field@ConnCreateParams::match_any_tag`] field.
    pub fn match_any_tag(&mut self, value: bool) -> &mut Self {
        self.match_any_tag = value;
        self
    }

    /// Add `value` to [`field@ConnCreateParams::sharding_key_columns`] field.
    pub fn sharding_key_column(&mut self, value: ShardingKeyColumn) -> &mut Self {
        self.sharding_key_columns.push(value);
        self
    }

    /// Add multiple values to [`field@ConnCreateParams::sharding_key_columns`] field.
    pub fn sharding_key_columns<T>(&mut self, values: T) -> &mut Self
    where
        T: IntoIterator<Item = ShardingKeyColumn>,
    {
        self.sharding_key_columns.extend(values);
        self
    }

    /// Add `value` to [`field@ConnCreateParams::super_sharding_key_columns`] field.
    pub fn super_sharding_key_column(&mut self, value: ShardingKeyColumn) -> &mut Self {
        self.super_sharding_key_columns.push(value);
        self
    }

    /// Add multiple values to [`field@ConnCreateParams::super_sharding_key_columns`] field.
    pub fn super_sharding_key_columns<T>(&mut self, values: T) -> &mut Self
    where
        T: IntoIterator<Item = ShardingKeyColumn>,
    {
        self.super_sharding_key_columns.extend(values);
        self
    }
}

pub(crate) struct DpiConnCreateParams<'a> {
    dpi_params: dpiConnCreateParams,
    params: Option<&'a mut ConnCreateParams>,
    _app_context: Vec<dpiAppContext>,
    _sharding_key_columns: Vec<dpiShardingKeyColumn>,
    _super_sharding_key_columns: Vec<dpiShardingKeyColumn>,
}

impl<'a> DpiConnCreateParams<'a> {
    pub(crate) fn new(
        opt_params: Option<&'a mut ConnCreateParams>,
    ) -> Result<DpiConnCreateParams<'a>> {
        if let Some(params) = &opt_params {
            let mut app_context = params
                .app_context
                .iter()
                .map(AppContext::try_to_dpi)
                .collect::<Result<Vec<_>>>()?;
            let mut sharding_key_columns = params
                .sharding_key_columns
                .iter()
                .map(ShardingKeyColumn::try_to_dpi)
                .collect::<Result<Vec<_>>>()?;
            let mut super_sharding_key_columns = params
                .super_sharding_key_columns
                .iter()
                .map(ShardingKeyColumn::try_to_dpi)
                .collect::<Result<Vec<_>>>()?;
            Ok(DpiConnCreateParams {
                dpi_params: dpiConnCreateParams {
                    authMode: params.auth_mode.to_dpi(),
                    connectionClass: params.connection_class.to_ptr(),
                    connectionClassLength: params.connection_class.try_to_len()?,
                    purity: params.purity.to_dpi(),
                    newPassword: params.new_password.to_ptr(),
                    newPasswordLength: params.new_password.try_to_len()?,
                    appContext: if app_context.is_empty() {
                        ptr::null_mut()
                    } else {
                        app_context.as_mut_ptr()
                    },
                    numAppContext: app_context.len().try_into()?,
                    externalAuth: params.external_auth.to_dpi(),
                    externalHandle: params.external_handle,
                    pool: params.pool.to_dpi(),
                    tag: params.tag.to_ptr(),
                    tagLength: params.tag.try_to_len()?,
                    matchAnyTag: params.match_any_tag.to_dpi(),
                    outTag: ptr::null(),
                    outTagLength: 0,
                    outTagFound: 0,
                    shardingKeyColumns: if sharding_key_columns.is_empty() {
                        ptr::null_mut()
                    } else {
                        sharding_key_columns.as_mut_ptr()
                    },
                    numShardingKeyColumns: sharding_key_columns.len().try_into()?,
                    superShardingKeyColumns: if super_sharding_key_columns.is_empty() {
                        ptr::null_mut()
                    } else {
                        super_sharding_key_columns.as_mut_ptr()
                    },
                    numSuperShardingKeyColumns: super_sharding_key_columns.len().try_into()?,
                    outNewSession: 0,
                },
                params: opt_params,
                _app_context: app_context,
                _sharding_key_columns: sharding_key_columns,
                _super_sharding_key_columns: super_sharding_key_columns,
            })
        } else {
            let ctxt = Context::get()?;
            let dpi_params = get_value!(dpiContext_initConnCreateParams(ctxt.handle))?;
            Ok(DpiConnCreateParams {
                dpi_params,
                params: opt_params,
                _app_context: Vec::new(),
                _sharding_key_columns: Vec::new(),
                _super_sharding_key_columns: Vec::new(),
            })
        }
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut dpiConnCreateParams {
        &mut self.dpi_params
    }

    pub(crate) fn update_out_params(&mut self) -> Result<()> {
        if let Some(params) = &mut self.params {
            let prms = &self.dpi_params;
            params.out_tag = if prms.outTagFound != 0 {
                Some((prms.outTag, prms.outTagLength).try_to_rust()?)
            } else {
                None
            };
            params.out_new_session = prms.outNewSession != 0;
        }
        Ok(())
    }
}
