use crate::stmt::Stmt;
use crate::types::{
    IntervalDS, IntervalYM, Json, Lob, NativeType, Object, Rowid, Timestamp, Vector,
};
use crate::*;
use odpic_sys::*;
use std::str;

#[derive(Debug)]
pub enum NativeValue<'a> {
    Int64(Option<i64>),
    Uint64(Option<u64>),
    Float(Option<f32>),
    Double(Option<f64>),
    Bytes(Option<&'a [u8]>),
    Timestamp(Option<Timestamp>),
    IntervalDS(Option<IntervalDS>),
    IntervalYM(Option<IntervalYM>),
    Lob(Option<Lob>),
    Object(Option<Object>),
    Stmt(Option<Stmt>),
    Boolean(Option<bool>),
    Rowid(Option<Rowid>),
    Json(Option<Json>),
    Vector(Option<Vector>),
}

impl NativeValue<'_> {
    pub(crate) fn from_dpi_data(data: &dpiData, ty: NativeType) -> Result<NativeValue> {
        fn to_opt<T>(data: &dpiData, value: T) -> Option<T> {
            if data.isNull == 0 {
                Some(value)
            } else {
                None
            }
        }
        let value = data.value;
        unsafe {
            Ok(match ty {
                NativeType::Int64 => NativeValue::Int64(to_opt(data, value.asInt64)),
                NativeType::Uint64 => NativeValue::Uint64(to_opt(data, value.asUint64)),
                NativeType::Float => NativeValue::Float(to_opt(data, value.asFloat)),
                NativeType::Double => NativeValue::Double(to_opt(data, value.asDouble)),
                NativeType::Bytes => NativeValue::Bytes(to_opt(
                    data,
                    slice::from_raw_parts(
                        value.asBytes.ptr as *mut u8,
                        value.asBytes.length as usize,
                    ),
                )),
                NativeType::Timestamp => {
                    NativeValue::Timestamp(to_opt(data, value.asTimestamp.into()))
                }
                NativeType::IntervalDS => {
                    NativeValue::IntervalDS(to_opt(data, value.asIntervalDS.into()))
                }
                NativeType::IntervalYM => {
                    NativeValue::IntervalYM(to_opt(data, value.asIntervalYM.into()))
                }
                NativeType::Lob => NativeValue::Lob(to_opt(data, Lob::with_add_ref(value.asLOB))),
                NativeType::Object => {
                    NativeValue::Object(to_opt(data, Object::with_add_ref(value.asObject)))
                }
                NativeType::Stmt => {
                    NativeValue::Stmt(to_opt(data, Stmt::with_add_ref(value.asStmt)))
                }
                NativeType::Boolean => NativeValue::Boolean(to_opt(data, value.asBoolean != 0)),
                NativeType::Rowid => {
                    NativeValue::Rowid(to_opt(data, Rowid::with_add_ref(value.asRowid)))
                }
                NativeType::Json => {
                    NativeValue::Json(to_opt(data, Json::with_add_ref(value.asJson)))
                }
                NativeType::Vector => {
                    NativeValue::Vector(to_opt(data, Vector::with_add_ref(value.asVector)))
                }
                NativeType::JsonObject | NativeType::JsonArray | NativeType::Null => {
                    return Err(Error::other(format!("unexpected native type {:?}", ty)));
                }
            })
        }
    }
}

impl<'a> TryFrom<NativeValue<'a>> for i64 {
    type Error = Error;

    fn try_from(value: NativeValue<'a>) -> Result<i64> {
        match value {
            NativeValue::Int64(Some(value)) => Ok(value),
            NativeValue::Int64(None) => Err(Error::NullValue),
            _ => Err(Error::other(format!(
                "failed to convert to &str from {:?}",
                value
            ))),
        }
    }
}

impl<'a> TryFrom<NativeValue<'a>> for u64 {
    type Error = Error;

    fn try_from(value: NativeValue<'a>) -> Result<u64> {
        match value {
            NativeValue::Uint64(Some(value)) => Ok(value),
            NativeValue::Uint64(None) => Err(Error::NullValue),
            _ => Err(Error::other(format!(
                "failed to convert to &str from {:?}",
                value
            ))),
        }
    }
}

impl<'a> TryFrom<NativeValue<'a>> for f32 {
    type Error = Error;

    fn try_from(value: NativeValue<'a>) -> Result<f32> {
        match value {
            NativeValue::Float(Some(value)) => Ok(value),
            NativeValue::Float(None) => Err(Error::NullValue),
            _ => Err(Error::other(format!(
                "failed to convert to &str from {:?}",
                value
            ))),
        }
    }
}

impl<'a> TryFrom<NativeValue<'a>> for f64 {
    type Error = Error;

    fn try_from(value: NativeValue<'a>) -> Result<f64> {
        match value {
            NativeValue::Double(Some(value)) => Ok(value),
            NativeValue::Double(None) => Err(Error::NullValue),
            _ => Err(Error::other(format!(
                "failed to convert to &str from {:?}",
                value
            ))),
        }
    }
}

impl<'a> TryFrom<NativeValue<'a>> for &'a str {
    type Error = Error;

    fn try_from(value: NativeValue<'a>) -> Result<&'a str> {
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

impl<'a> TryFrom<NativeValue<'a>> for &'a [u8] {
    type Error = Error;

    fn try_from(value: NativeValue<'a>) -> Result<&'a [u8]> {
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

impl TryFrom<NativeValue<'_>> for Rowid {
    type Error = Error;

    fn try_from(value: NativeValue<'_>) -> Result<Rowid> {
        match value {
            NativeValue::Rowid(Some(rowid)) => Ok(rowid),
            NativeValue::Rowid(None) => Err(Error::NullValue),
            _ => Err(Error::other(format!(
                "failed to convert to &str from {:?}",
                value
            ))),
        }
    }
}

impl<'a, T> TryFrom<NativeValue<'a>> for Option<T>
where
    T: TryFrom<NativeValue<'a>, Error = Error>,
{
    type Error = Error;

    fn try_from(value: NativeValue<'a>) -> Result<Option<T>> {
        match <T as TryFrom<NativeValue<'a>>>::try_from(value) {
            Ok(value) => Ok(Some(value)),
            Err(Error::NullValue) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

impl From<i64> for NativeValue<'static> {
    fn from(value: i64) -> NativeValue<'static> {
        NativeValue::Int64(Some(value))
    }
}

impl From<Option<i64>> for NativeValue<'static> {
    fn from(value: Option<i64>) -> NativeValue<'static> {
        NativeValue::Int64(value)
    }
}
