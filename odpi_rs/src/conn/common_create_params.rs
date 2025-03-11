use crate::conn::AccessToken;
use crate::context::Context;
use crate::utils::*;
use crate::*;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::marker::PhantomData;
use std::ptr;

#[derive(Debug, Clone)]
#[non_exhaustive]
#[odpic_doc]
pub struct CommonCreateParams {
    pub create_mode: CreateMode,
    pub edition: String,
    pub driver_name: String,
    pub soda_metadata_cache: bool,
    pub stmt_cache_size: u32,
    pub access_token: Option<AccessToken>,
}

impl CommonCreateParams {
    /// Create `CommonCreateParams`
    ///
    /// # Note
    /// This intanally calls [`Context::get()`].
    ///
    /// If you need to call [`Context::init()`], it must be called before this.
    pub fn new() -> Result<CommonCreateParams> {
        let ctxt = Context::get()?;
        let params = get_value!(dpiContext_initCommonCreateParams(ctxt.handle))?;
        Ok(CommonCreateParams {
            create_mode: CreateMode::from_bits_truncate(params.createMode),
            edition: (params.edition, params.editionLength).try_to_rust()?,
            driver_name: (params.driverName, params.driverNameLength).try_to_rust()?,
            soda_metadata_cache: params.sodaMetadataCache != 0,
            stmt_cache_size: params.stmtCacheSize,
            access_token: None,
        })
    }

    /// Sets `value` to [`field@CommonCreateParams::create_mode`] field.
    pub fn create_mode(&mut self, value: CreateMode) -> &mut Self {
        self.create_mode = value;
        self
    }

    /// Sets `value` to [`field@CommonCreateParams::edition`] field.
    pub fn edition<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.edition = value.into();
        self
    }

    /// Sets `value` to [`field@CommonCreateParams::driver_name`] field.
    pub fn driver_name<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.driver_name = value.into();
        self
    }

    /// Sets `value` to [`field@CommonCreateParams::soda_metadata_cache`] field.
    pub fn soda_metadata_cache(&mut self, value: bool) -> &mut Self {
        self.soda_metadata_cache = value;
        self
    }

    /// Sets `value` to [`field@CommonCreateParams::stmt_cache_size`] field.
    pub fn stmt_cache_size<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<u32>,
    {
        self.stmt_cache_size = value.into();
        self
    }

    /// Sets `value` to [`field@CommonCreateParams::access_token`] field.
    pub fn access_token(&mut self, value: AccessToken) -> &mut Self {
        self.access_token = Some(value);
        self
    }
}

pub(crate) struct DpiCommonCreateParams<'a> {
    params: dpiCommonCreateParams,
    access_token: Option<dpiAccessToken>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> DpiCommonCreateParams<'a> {
    pub(crate) fn new(
        params: &Option<&'a CommonCreateParams>,
    ) -> Result<DpiCommonCreateParams<'a>> {
        if let Some(params) = params {
            Ok(DpiCommonCreateParams {
                params: dpiCommonCreateParams {
                    createMode: params.create_mode.bits() | DPI_MODE_CREATE_THREADED,
                    encoding: ptr::null(),
                    nencoding: ptr::null(),
                    edition: params.edition.to_ptr(),
                    editionLength: params.edition.try_to_len()?,
                    driverName: params.driver_name.to_ptr(),
                    driverNameLength: params.driver_name.try_to_len()?,
                    sodaMetadataCache: params.soda_metadata_cache.to_dpi(),
                    stmtCacheSize: params.stmt_cache_size,
                    accessToken: ptr::null_mut(), // updated in DpiCommonCreateParams::as_ref()
                },
                access_token: params
                    .access_token
                    .as_ref()
                    .map(AccessToken::try_to_dpi)
                    .transpose()?,
                phantom: PhantomData,
            })
        } else {
            let ctxt = Context::get()?;
            let mut params = get_value!(dpiContext_initCommonCreateParams(ctxt.handle))?;
            params.createMode |= DPI_MODE_CREATE_THREADED;
            Ok(DpiCommonCreateParams {
                params,
                access_token: None,
                phantom: PhantomData,
            })
        }
    }

    pub(crate) fn as_ptr(&mut self) -> *const dpiCommonCreateParams {
        self.params.accessToken = if let Some(token) = &mut self.access_token {
            token
        } else {
            ptr::null_mut()
        };
        &self.params
    }
}
