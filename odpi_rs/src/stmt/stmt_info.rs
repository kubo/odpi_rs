use crate::stmt::StatementType;
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug, Clone)]
#[non_exhaustive]
#[odpic_doc]
pub struct StmtInfo {
    pub is_query: bool,
    pub is_plsql: bool,
    pub is_ddl: bool,
    pub is_dml: bool,
    pub statement_type: StatementType,
    pub is_returning: bool,
}

impl TryToRust<StmtInfo> for dpiStmtInfo {
    fn try_to_rust(&self) -> Result<StmtInfo> {
        Ok(StmtInfo {
            is_query: self.isQuery.to_rust(),
            is_plsql: self.isPLSQL.to_rust(),
            is_ddl: self.isDDL.to_rust(),
            is_dml: self.isDML.to_rust(),
            statement_type: self.statementType.try_to_rust()?,
            is_returning: self.isReturning.to_rust(),
        })
    }
}
