use crate::utils::*;
use crate::*;
use odpic_sys::*;
use std::convert::Infallible;
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError, TryFromIntError};
use std::result;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("{source}")]
    #[non_exhaustive]
    OdpiError {
        #[from]
        source: OdpiError,
    },
    #[error("{source}")]
    #[non_exhaustive]
    TryFromIntError {
        #[from]
        source: TryFromIntError,
    },
    #[error("{source}")]
    #[non_exhaustive]
    ParseFloatError {
        #[from]
        source: ParseFloatError,
    },
    #[error("{source}")]
    #[non_exhaustive]
    ParseIntError {
        #[from]
        source: ParseIntError,
    },
    #[error("{source}")]
    #[non_exhaustive]
    Utf8Erorr {
        #[from]
        source: Utf8Error,
    },
    #[error("null value found")]
    NullValue,
    #[error("{message}")]
    #[non_exhaustive]
    Other { message: String },
}

impl Error {
    pub(crate) fn other<T>(message: T) -> Error
    where
        T: Into<String>,
    {
        Error::Other {
            message: message.into(),
        }
    }
}

impl From<Infallible> for Error {
    fn from(_value: Infallible) -> Error {
        unreachable!()
    }
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct OdpiError {
    pub code: i32,
    pub message: String,
    pub fn_name: &'static str,
    pub action: &'static str,
    pub sql_state: &'static str,
    pub is_recoverable: bool,
    pub is_warning: bool,
    pub offset: u32,
}

impl OdpiError {
    pub fn from_dpi(err: &dpiErrorInfo) -> OdpiError {
        let err_bytes: &[u8] = (err.message, err.messageLength).try_to_rust().unwrap();
        OdpiError {
            code: err.code,
            message: String::from_utf8_lossy(err_bytes).into_owned(),
            fn_name: unsafe { CStr::from_ptr(err.fnName) }.to_str().unwrap_or(""),
            action: unsafe { CStr::from_ptr(err.action) }.to_str().unwrap_or(""),
            sql_state: unsafe { CStr::from_ptr(err.sqlState) }
                .to_str()
                .unwrap_or(""),
            is_recoverable: err.isRecoverable != 0,
            is_warning: err.isWarning != 0,
            offset: err.offset,
        }
    }
}

impl fmt::Display for OdpiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        f.write_str(&self.message)
    }
}

impl error::Error for OdpiError {}
