use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Clone, Debug)]
#[odpic_doc]
pub struct AppContext {
    pub namespace_name: String,
    pub name: String,
    pub value: String,
}

impl AppContext {
    pub fn new<NS, N, V>(namespace_name: NS, name: N, value: V) -> AppContext
    where
        NS: Into<String>,
        N: Into<String>,
        V: Into<String>,
    {
        AppContext {
            namespace_name: namespace_name.into(),
            name: name.into(),
            value: value.into(),
        }
    }
}

impl TryToDpi<dpiAppContext> for AppContext {
    fn try_to_dpi(&self) -> Result<dpiAppContext> {
        Ok(dpiAppContext {
            namespaceName: self.namespace_name.to_ptr(),
            namespaceNameLength: self.namespace_name.try_to_len()?,
            name: self.name.to_ptr(),
            nameLength: self.name.try_to_len()?,
            value: self.value.to_ptr(),
            valueLength: self.value.try_to_len()?,
        })
    }
}
