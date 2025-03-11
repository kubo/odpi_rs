use crate::types::{NativeValue, Rowid};
use crate::{Error, Result};
use std::str::{self, FromStr};

pub trait FromSql: Sized {
    fn from_sql(value: NativeValue<'_>) -> Result<Self>;
}

impl<T> FromSql for Option<T>
where
    T: FromSql,
{
    fn from_sql(value: NativeValue<'_>) -> Result<Option<T>> {
        match <T as FromSql>::from_sql(value) {
            Ok(value) => Ok(Some(value)),
            Err(Error::NullValue) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

impl FromSql for bool {
    fn from_sql(value: NativeValue<'_>) -> Result<bool> {
        match value {
            NativeValue::Boolean(Some(value)) => Ok(value),
            NativeValue::Boolean(None) => Err(Error::NullValue),
            _ => Err(Error::other(format!(
                "failed to convert to bool from {:?}",
                value
            ))),
        }
    }
}

macro_rules! from_sql_num_impl {
    ($kind:ident : $($t:ty)*) => {$(
        impl FromSql for $t {
            fn from_sql(value: NativeValue<'_>) -> Result<$t> {
                match value {
                    NativeValue::Bytes(Some(bytes)) => Ok(<$t>::from_str(
                        unsafe { str::from_utf8_unchecked(bytes) }
                    )?),
                    NativeValue::Bytes(None) => Err(Error::NullValue),
                    NativeValue::Int64(Some(value)) => Ok(from_sql_num_impl!(value => $kind $t)),
                    NativeValue::Int64(None) => Err(Error::NullValue),
                    NativeValue::Uint64(Some(value)) => Ok(from_sql_num_impl!(value => $kind $t)),
                    NativeValue::Uint64(None) => Err(Error::NullValue),
                    _ => Err(Error::other(format!(
                        concat!("failed to convert to ", stringify!($t), " from {:?}"),
                        value
                    ))),
                }
            }
        }
    )*};
    ($value:expr => int $t:ty) => { $value.try_into()? };
    ($value:expr => float $t:ty) => { $value as $t };
}

from_sql_num_impl! { int: isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128 }
from_sql_num_impl! { float: f32 f64 }

impl FromSql for Rowid {
    fn from_sql(value: NativeValue<'_>) -> Result<Rowid> {
        match value {
            NativeValue::Rowid(Some(rowid)) => Ok(rowid),
            NativeValue::Rowid(None) => Err(Error::NullValue),
            _ => Err(Error::other(format!(
                "failed to convert to Rowid from {:?}",
                value
            ))),
        }
    }
}

pub trait FromSqlUnsafe<'a>: Sized {
    unsafe fn from_sql_unsafe(value: NativeValue<'a>) -> Result<Self>;
}

impl<'a, T> FromSqlUnsafe<'a> for Option<T>
where
    T: FromSqlUnsafe<'a>,
{
    unsafe fn from_sql_unsafe(value: NativeValue<'a>) -> Result<Option<T>> {
        match <T as FromSqlUnsafe<'a>>::from_sql_unsafe(value) {
            Ok(value) => Ok(Some(value)),
            Err(Error::NullValue) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

impl<'a> FromSqlUnsafe<'a> for NativeValue<'a> {
    unsafe fn from_sql_unsafe(value: NativeValue<'a>) -> Result<NativeValue<'a>> {
        Ok(value)
    }
}

impl<'a> FromSqlUnsafe<'a> for &'a str {
    unsafe fn from_sql_unsafe(value: NativeValue<'a>) -> Result<&'a str> {
        match value {
            NativeValue::Bytes(Some(bytes)) => Ok(str::from_utf8(bytes)?),
            NativeValue::Bytes(None) => Err(Error::NullValue),
            _ => Err(Error::other(format!(
                "failed to convert to &str from {:?}",
                value
            ))),
        }
    }
}

impl<'a> FromSqlUnsafe<'a> for &'a [u8] {
    unsafe fn from_sql_unsafe(value: NativeValue<'a>) -> Result<&'a [u8]> {
        match value {
            NativeValue::Bytes(Some(bytes)) => Ok(bytes),
            NativeValue::Bytes(None) => Err(Error::NullValue),
            _ => Err(Error::other(format!(
                "failed to convert to &str from {:?}",
                value
            ))),
        }
    }
}
