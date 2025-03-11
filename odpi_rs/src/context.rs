use crate::*;
use odpic_sys::*;
use once_cell::sync::OnceCell;
use std::mem::MaybeUninit;
use std::ptr;

mod context_create_params;
pub use context_create_params::ContextCreateParams;

pub struct Context {
    pub(crate) handle: *mut dpiContext,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

static GLOBAL_CONTEXT: OnceCell<Context> = OnceCell::new();

impl Context {
    fn new(params: &mut ContextCreateParams) -> Result<Context> {
        let mut handle = ptr::null_mut();
        let mut dpi_params = params.to_dpi();
        unsafe {
            let mut err = MaybeUninit::uninit();
            if dpiContext_createWithParams(
                DPI_MAJOR_VERSION,
                DPI_MINOR_VERSION,
                &mut dpi_params,
                &mut handle,
                err.as_mut_ptr(),
            ) != 0
            {
                return Err(OdpiError::from_dpi(&err.assume_init()).into());
            }
        }
        params.set_dpi_params(&dpi_params);
        Ok(Context { handle })
    }

    pub fn get() -> Result<&'static Context> {
        GLOBAL_CONTEXT.get_or_try_init(|| {
            let mut params = ContextCreateParams::new();
            Context::new(&mut params)
        })
    }

    pub fn init(params: &mut ContextCreateParams) -> Result<bool> {
        let mut initialized_here = false;
        GLOBAL_CONTEXT.get_or_try_init(|| {
            let ctxt = Context::new(params);
            initialized_here = true;
            ctxt
        })?;
        Ok(initialized_here)
    }

    pub fn client_version(&self) -> Result<VersionInfo> {
        Ok(get_value!(dpiContext_getClientVersion(self.handle))?.into())
    }

    pub fn last_error(&self) -> Error {
        unsafe {
            let mut err = MaybeUninit::uninit();
            dpiContext_getError(self.handle, err.as_mut_ptr());
            OdpiError::from_dpi(&err.assume_init()).into()
        }
    }
}
