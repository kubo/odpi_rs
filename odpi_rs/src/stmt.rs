// odpi_rs - a thin wrapper over Oracle Database Programming Interface for C
//
// URL: https://github.com/kubo/odpi_rs
//
//-----------------------------------------------------------------------------
// Copyright (c) 2025 Kubo Takehiro <kubo@jiubao.org>. All rights reserved.
// This program is free software: you can modify it and/or redistribute it
// under the terms of:
//
// (i)  the Universal Permissive License v 1.0 or at your option, any
//      later version (http://oss.oracle.com/licenses/upl); and/or
//
// (ii) the Apache License v 2.0. (http://www.apache.org/licenses/LICENSE-2.0)
//-----------------------------------------------------------------------------
//! Types for statement executions and queries

use crate::private;
use crate::types::{
    FromSql, FromSqlUnsafe, IntervalDS, IntervalYM, NativeType, NativeValue, Object, ObjectType,
    OracleType, Rowid, Timestamp,
};
use crate::utils::*;
use crate::*;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::ffi::c_void;
use std::mem;
use std::ops::Range;

mod enums;
mod query_info;
mod stmt_info;
mod var;

pub use enums::*;
pub use query_info::QueryInfo;
pub use stmt_info::StmtInfo;
pub use var::Var;

/// Result of [`Stmt::fetch_rows`]
#[derive(Clone, Debug)]
pub struct FetchRowsResult {
    pub indexes: Range<u32>,
    pub more_rows: bool,
}

/// Value passed to [`Stmt::bind_value_by_name`] and [`Stmt::bind_value_by_pos`]
pub trait BindValue: private::Sealed {
    #[doc(hidden)]
    fn native_type_num() -> dpiNativeTypeNum;
    #[doc(hidden)]
    fn data(&self) -> Result<dpiData> {
        let mut data: dpiData = unsafe { mem::zeroed() };
        self.set_to_data(&mut data)?;
        Ok(data)
    }
    #[doc(hidden)]
    fn set_to_data(&self, data: &mut dpiData) -> Result<()>;
}

impl<T> BindValue for Option<T>
where
    T: BindValue,
{
    fn native_type_num() -> dpiNativeTypeNum {
        T::native_type_num()
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        if let Some(value) = self {
            value.set_to_data(data)
        } else {
            data.isNull = 1;
            Ok(())
        }
    }
}

impl BindValue for i8 {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_INT64
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asInt64 = (*self).into();
        Ok(())
    }
}

impl BindValue for i16 {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_INT64
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asInt64 = (*self).into();
        Ok(())
    }
}

impl BindValue for i32 {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_INT64
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asInt64 = (*self).into();
        Ok(())
    }
}

impl BindValue for i64 {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_INT64
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asInt64 = *self;
        Ok(())
    }
}

impl BindValue for u8 {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_UINT64
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asUint64 = (*self).into();
        Ok(())
    }
}

impl BindValue for u16 {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_UINT64
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asUint64 = (*self).into();
        Ok(())
    }
}

impl BindValue for u32 {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_UINT64
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asUint64 = (*self).into();
        Ok(())
    }
}

impl BindValue for u64 {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_UINT64
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asUint64 = *self;
        Ok(())
    }
}

impl BindValue for f32 {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_FLOAT
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asFloat = *self;
        Ok(())
    }
}

impl BindValue for f64 {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_DOUBLE
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asDouble = *self;
        Ok(())
    }
}

impl BindValue for &str {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_BYTES
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asBytes.ptr = self.as_ptr() as *mut c_char;
        data.value.asBytes.length = self.len().try_into()?;
        Ok(())
    }
}

impl BindValue for Timestamp {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_TIMESTAMP
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asTimestamp = (*self).into();
        Ok(())
    }
}

impl BindValue for IntervalYM {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_INTERVAL_YM
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asIntervalYM = (*self).into();
        Ok(())
    }
}

impl BindValue for IntervalDS {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_INTERVAL_DS
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asIntervalDS = (*self).into();
        Ok(())
    }
}

impl BindValue for Object {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_OBJECT
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asObject = self.handle;
        Ok(())
    }
}

impl BindValue for Rowid {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_ROWID
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asRowid = self.handle;
        Ok(())
    }
}

impl BindValue for bool {
    fn native_type_num() -> dpiNativeTypeNum {
        DPI_NATIVE_TYPE_BOOLEAN
    }
    fn set_to_data(&self, data: &mut dpiData) -> Result<()> {
        data.value.asBoolean = (*self).into();
        Ok(())
    }
}

#[derive(Debug)]
#[odpic_doc]
pub struct Stmt {
    pub(crate) handle: *mut dpiStmt,
}

#[odpic_doc]
impl Stmt {
    pub(crate) fn new(handle: *mut dpiStmt) -> Stmt {
        Stmt { handle }
    }

    pub(crate) fn with_add_ref(handle: *mut dpiStmt) -> Stmt {
        unsafe { dpiStmt_addRef(handle) };
        Stmt { handle }
    }

    pub fn bind_by_name<T>(&self, name: T, var: &Var) -> Result<()>
    where
        T: AsRef<str>,
    {
        call!(dpiStmt_bindByName(
            self.handle,
            name.to_ptr(),
            name.try_to_len()?,
            var.handle
        ))
    }

    pub fn bind_by_pos(&self, pos: u32, var: &Var) -> Result<()> {
        call!(dpiStmt_bindByPos(self.handle, pos, var.handle))
    }

    pub fn bind_value_by_name<N, T>(&self, name: N, value: &T) -> Result<()>
    where
        N: AsRef<str>,
        T: BindValue,
    {
        call!(dpiStmt_bindValueByName(
            self.handle,
            name.to_ptr(),
            name.try_to_len()?,
            T::native_type_num(),
            &mut value.data()?,
        ))
    }

    pub fn bind_value_by_pos<T>(&self, pos: u32, value: &T) -> Result<()>
    where
        T: BindValue,
    {
        call!(dpiStmt_bindValueByPos(
            self.handle,
            pos,
            T::native_type_num(),
            &mut value.data()?,
        ))
    }

    pub fn close<T>(&self, tag: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        call!(dpiStmt_close(self.handle, tag.to_ptr(), tag.try_to_len()?))
    }

    pub fn define(&self, pos: u32, var: &Var) -> Result<()> {
        call!(dpiStmt_define(self.handle, pos, var.handle))
    }

    pub fn define_value(
        &self,
        pos: u32,
        oracle_type: OracleType,
        native_type: NativeType,
        size: u32,
        size_is_bytes: bool,
        obj_type: Option<&ObjectType>,
    ) -> Result<()> {
        call!(dpiStmt_defineValue(
            self.handle,
            pos,
            oracle_type.to_dpi(),
            native_type.to_dpi(),
            size,
            size_is_bytes.to_dpi(),
            obj_type.to_dpi()
        ))
    }

    pub fn delete_from_cache(&self) -> Result<()> {
        call!(dpiStmt_deleteFromCache(self.handle))
    }

    #[maybe_async]
    pub async fn execute(&self, mode: ExecMode) -> Result<u32> {
        Ok(*get_value_blocking! {
            let handle = self.handle;
            dpiStmt_execute(*handle, mode.bits())
        }
        .await?)
    }

    #[maybe_async]
    pub async fn execute_many(&self, mode: ExecMode, num_iters: u32) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            dpiStmt_executeMany(*handle, mode.to_dpi(), num_iters)
        }
        .await
    }

    #[maybe_async]
    pub async fn fetch(&self) -> Result<Option<u32>> {
        let (found, buffer_row_index) = get_2values_blocking! {
            let handle = self.handle;
            dpiStmt_fetch(*handle)
        }
        .await?;
        Ok(if *found != 0 {
            Some(*buffer_row_index)
        } else {
            None
        })
    }

    #[maybe_async]
    pub async fn fetch_rows(&self, max_rows: u32) -> Result<FetchRowsResult> {
        let (buffer_row_index, num_rows_fetched, more_rows) = get_3values_blocking! {
            let handle = self.handle;
            dpiStmt_fetchRows(*handle, max_rows)
        }
        .await?;
        Ok(FetchRowsResult {
            indexes: *buffer_row_index..(*buffer_row_index + *num_rows_fetched),
            more_rows: more_rows.to_rust(),
        })
    }

    pub fn batch_error_count(&self) -> Result<u32> {
        get_value!(dpiStmt_getBatchErrorCount(self.handle))
    }

    pub fn batch_errors(&self) -> Result<Vec<Error>> {
        let count = self.batch_error_count()?;
        let usize_count = count.try_into()?;
        let mut errors = Vec::with_capacity(usize_count);
        call!(dpiStmt_getBatchErrors(
            self.handle,
            count,
            errors.as_mut_ptr()
        ))?;
        unsafe {
            errors.set_len(usize_count);
        }
        Ok(errors
            .iter()
            .map(OdpiError::from_dpi)
            .map(Error::from)
            .collect())
    }

    pub fn bind_count(&self) -> Result<u32> {
        get_value!(dpiStmt_getBindCount(self.handle))
    }

    pub fn bind_names(&self) -> Result<Vec<String>> {
        let mut count = self.bind_count()?;
        if count == 0 {
            return Ok(vec![]);
        }
        let usize_count = count.try_into()?;
        let mut names = Vec::with_capacity(usize_count);
        let mut name_lenghts = Vec::with_capacity(usize_count);
        call!(dpiStmt_getBindNames(
            self.handle,
            &mut count,
            names.as_mut_ptr(),
            name_lenghts.as_mut_ptr(),
        ))?;
        let usize_count = count.try_into()?;
        unsafe {
            names.set_len(usize_count);
            name_lenghts.set_len(usize_count);
        }
        names
            .into_iter()
            .zip(name_lenghts)
            .map(|(name, len)| (name, len).try_to_rust())
            .collect::<Result<Vec<_>>>()
    }

    pub fn fetch_array_size(&self) -> Result<u32> {
        get_value!(dpiStmt_getFetchArraySize(self.handle))
    }

    pub fn implicit_result(&self) -> Result<Option<Stmt>> {
        let handle = get_value!(dpiStmt_getImplicitResult(self.handle))?;
        Ok(if handle.is_null() {
            None
        } else {
            Some(Stmt::new(handle))
        })
    }

    pub fn info(&self) -> Result<StmtInfo> {
        get_value!(dpiStmt_getInfo(self.handle))?.try_to_rust()
    }

    pub fn last_rowid(&self) -> Result<Option<Rowid>> {
        let handle = get_value!(dpiStmt_getLastRowid(self.handle))?;
        Ok(if handle.is_null() {
            None
        } else {
            Some(Rowid::with_add_ref(handle))
        })
    }

    pub fn num_query_columns(&self) -> Result<u32> {
        get_value!(dpiStmt_getNumQueryColumns(self.handle))
    }

    pub unsafe fn oci_attr(&self, attribute: u32) -> Result<(*const c_void, u32)> {
        let (buffer, len) = get_2values!(dpiStmt_getOciAttr(self.handle, attribute))?;
        Ok((buffer.asRaw, len))
    }

    pub fn prefetch_rows(&self) -> Result<u32> {
        get_value!(dpiStmt_getPrefetchRows(self.handle))
    }

    pub fn query_info(&self, pos: u32) -> Result<QueryInfo> {
        get_value!(dpiStmt_getQueryInfo(self.handle, pos))?.try_to_rust()
    }

    pub fn query_value<T>(&self, pos: u32) -> Result<T>
    where
        T: FromSql,
    {
        let (native_type_num, data) = get_2values!(dpiStmt_getQueryValue(self.handle, pos))?;
        let native_type = native_type_num.try_to_rust()?;
        <T as FromSql>::from_sql(NativeValue::from_dpi_data(unsafe { &*data }, native_type)?)
    }

    pub unsafe fn query_value_unsafe<'a, T>(&'a self, pos: u32) -> Result<T>
    where
        T: FromSqlUnsafe<'a>,
    {
        let (native_type_num, data) = get_2values!(dpiStmt_getQueryValue(self.handle, pos))?;
        let native_type = native_type_num.try_to_rust()?;
        <T as FromSqlUnsafe>::from_sql_unsafe(NativeValue::from_dpi_data(
            unsafe { &*data },
            native_type,
        )?)
    }

    pub fn row_count(&self) -> Result<u64> {
        get_value!(dpiStmt_getRowCount(self.handle))
    }

    pub fn row_counts(&self) -> Result<Vec<u64>> {
        let (num_row_counts, row_counts) = get_2values!(dpiStmt_getRowCounts(self.handle))?;
        Ok(unsafe { slice::from_raw_parts(row_counts, num_row_counts.try_into()?) }.into())
    }

    pub fn subscr_query_id(&self) -> Result<u64> {
        get_value!(dpiStmt_getSubscrQueryId(self.handle))
    }

    #[maybe_async]
    pub async fn scroll(&self, mode: FetchMode, offset: i32, row_count_offset: i32) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            dpiStmt_scroll(
                *handle,
                mode.to_dpi(),
                offset,
                row_count_offset
            )
        }
        .await
    }

    pub fn set_fetch_array_size(&self, array_size: u32) -> Result<()> {
        call!(dpiStmt_setFetchArraySize(self.handle, array_size))
    }

    pub unsafe fn set_oci_attr(
        &self,
        attribute: u32,
        value: *mut c_void,
        value_length: u32,
    ) -> Result<()> {
        call!(dpiStmt_setOciAttr(
            self.handle,
            attribute,
            value,
            value_length
        ))
    }

    pub fn set_prefetch_rows(&self, num_rows: u32) -> Result<()> {
        call!(dpiStmt_setPrefetchRows(self.handle, num_rows))
    }
}

impl Clone for Stmt {
    fn clone(&self) -> Stmt {
        unsafe { dpiStmt_addRef(self.handle) };
        Stmt {
            handle: self.handle,
        }
    }
}

impl Drop for Stmt {
    fn drop(&mut self) {
        release_handle!(dpiStmt_release(self.handle));
    }
}
