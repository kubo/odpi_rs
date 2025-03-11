use crate::conn::ServerType;
use crate::utils::*;
use crate::*;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug, Clone)]
#[non_exhaustive]
#[odpic_doc]
pub struct ConnInfo {
    pub db_domain: String,
    pub db_name: String,
    pub instance_name: String,
    pub service_name: String,
    pub max_identifier_length: u32,
    pub max_open_cursors: u32,
    pub server_type: ServerType,
}

impl TryToRust<ConnInfo> for dpiConnInfo {
    fn try_to_rust(&self) -> Result<ConnInfo> {
        Ok(ConnInfo {
            db_domain: (self.dbDomain, self.dbDomainLength).try_to_rust()?,
            db_name: (self.dbName, self.dbNameLength).try_to_rust()?,
            instance_name: (self.instanceName, self.instanceNameLength).try_to_rust()?,
            service_name: (self.serviceName, self.serviceNameLength).try_to_rust()?,
            max_identifier_length: self.maxIdentifierLength,
            max_open_cursors: self.maxOpenCursors,
            server_type: self.serverType.try_to_rust()?,
        })
    }
}
