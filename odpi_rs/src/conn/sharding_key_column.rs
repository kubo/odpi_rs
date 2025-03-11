use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::ffi::c_char;
use std::ptr;

#[derive(Clone, Debug)]
#[odpic_doc]
///
/// This enum has only one variant `Varchar` currently because
/// [`dpiShardingKeyColumn::oracleTypeNum`] supports `DPI_ORACLE_TYPE_VARCHAR` only at present.
#[non_exhaustive]
pub enum ShardingKeyColumn {
    Varchar(String),
}

impl TryToDpi<dpiShardingKeyColumn> for ShardingKeyColumn {
    fn try_to_dpi(&self) -> Result<dpiShardingKeyColumn> {
        match self {
            ShardingKeyColumn::Varchar(key) => Ok(dpiShardingKeyColumn {
                oracleTypeNum: DPI_ORACLE_TYPE_VARCHAR,
                nativeTypeNum: DPI_NATIVE_TYPE_BYTES,
                value: dpiDataBuffer {
                    asBytes: dpiBytes {
                        ptr: key.as_ptr() as *mut c_char,
                        length: key.len().try_into()?,
                        encoding: ptr::null(),
                    },
                },
            }),
        }
    }
}
