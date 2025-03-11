use crate::types::DataTypeInfo;
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug, Clone)]
#[non_exhaustive]
#[odpic_doc]
pub struct QueryInfo {
    pub name: String,
    pub type_info: DataTypeInfo,
    pub null_ok: bool,
}

impl TryToRust<QueryInfo> for dpiQueryInfo {
    fn try_to_rust(&self) -> Result<QueryInfo> {
        Ok(QueryInfo {
            name: (self.name, self.nameLength).try_to_rust()?,
            type_info: self.typeInfo.try_to_rust()?,
            null_ok: self.nullOk.to_rust(),
        })
    }
}
