use crate::stmt::Stmt;
use crate::types::{
    FromSql, FromSqlUnsafe, Json, Lob, NativeType, NativeValue, Object, Rowid, Vector,
};
use crate::utils::*;
use crate::*;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
#[odpic_doc]
pub struct Var {
    pub(crate) handle: *mut dpiVar,
    pub(crate) native_type: NativeType,
    pub(crate) data: Arc<Mutex<*mut dpiData>>,
}

#[odpic_doc]
impl Var {
    pub(crate) fn new(handle: *mut dpiVar, native_type: NativeType, data: *mut dpiData) -> Var {
        Var {
            handle,
            native_type,
            data: Arc::new(Mutex::new(data)),
        }
    }

    pub fn value<T>(&self) -> Result<T>
    where
        T: FromSql,
    {
        let data = self.data.lock().unwrap();
        <T as FromSql>::from_sql(NativeValue::from_dpi_data(
            unsafe { &**data },
            self.native_type,
        )?)
    }

    pub unsafe fn value_unsafe<'a, T>(&'a self) -> Result<T>
    where
        T: FromSqlUnsafe<'a>,
    {
        let data = self.data.lock().unwrap();
        <T as FromSqlUnsafe>::from_sql_unsafe(NativeValue::from_dpi_data(
            unsafe { &**data },
            self.native_type,
        )?)
    }

    pub fn copy_data(&self, pos: u32, source: &Var, source_pos: u32) -> Result<()> {
        call!(dpiVar_copyData(self.handle, pos, source.handle, source_pos))
    }

    pub fn num_elements_in_array(&self) -> Result<u32> {
        get_value!(dpiVar_getNumElementsInArray(self.handle))
    }

    // // return pointer to array of dpiData structures for transferring data
    // // this is needed for DML returning where the number of elements is modified
    // DPI_EXPORT int dpiVar_getReturnedData(dpiVar *var, uint32_t pos,
    //         uint32_t *numElements, dpiData **data);

    pub fn size_in_bytes(&self) -> Result<u32> {
        get_value!(dpiVar_getSizeInBytes(self.handle))
    }

    #[maybe_async]
    pub async fn set_from_bytes<T>(&self, pos: u32, value: T) -> Result<()>
    where
        T: AsRef<[u8]>,
    {
        call_blocking! {
            let handle = self.handle;
            let value_ptr = value.as_ref().as_ptr().to_dpi();
            let value_len = value.as_ref().len().try_into()?;
            dpiVar_setFromBytes(
                *handle,
                pos,
                *value_ptr,
                *value_len,
            )
        }
        .await
    }

    pub fn set_from_json(&self, pos: u32, value: &Json) -> Result<()> {
        call!(dpiVar_setFromJson(self.handle, pos, value.handle))
    }

    pub fn set_from_lob(&self, pos: u32, value: &Lob) -> Result<()> {
        call!(dpiVar_setFromLob(self.handle, pos, value.handle))
    }

    pub fn set_from_object(&self, pos: u32, value: &Object) -> Result<()> {
        call!(dpiVar_setFromObject(self.handle, pos, value.handle))
    }

    pub fn set_from_rowid(&self, pos: u32, value: &Rowid) -> Result<()> {
        call!(dpiVar_setFromRowid(self.handle, pos, value.handle))
    }

    pub fn set_from_stmt(&self, pos: u32, value: &Stmt) -> Result<()> {
        call!(dpiVar_setFromStmt(self.handle, pos, value.handle))
    }

    pub fn set_from_vector(&self, pos: u32, value: &Vector) -> Result<()> {
        call!(dpiVar_setFromVector(self.handle, pos, value.handle))
    }

    pub fn set_num_elements_in_array(&self, num_elements: u32) -> Result<()> {
        call!(dpiVar_setNumElementsInArray(self.handle, num_elements))
    }
}

impl Clone for Var {
    fn clone(&self) -> Var {
        unsafe { dpiVar_addRef(self.handle) };
        Var {
            handle: self.handle,
            native_type: self.native_type,
            data: self.data.clone(),
        }
    }
}

impl Drop for Var {
    fn drop(&mut self) {
        release_handle!(dpiVar_release(self.handle));
    }
}
