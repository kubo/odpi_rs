use crate::types::json::{Array, Object};
use crate::types::{IntervalDS, IntervalYM, NativeType, Timestamp};
use crate::{Error, Result};
use odpic_sys::*;
use std::slice;

pub enum NativeValue<'a> {
    Null,
    Boolean(bool),
    Bytes(&'a [u8]),
    Array(&'a Array<'a>),
    Object(&'a Object<'a>),
    Float(f32),
    Double(f64),
    Timestamp(Timestamp),
    IntervalDS(IntervalDS),
    IntervalYM(IntervalYM),
}

impl NativeValue<'_> {
    pub(crate) fn from_dpi(value: &dpiDataBuffer, ty: NativeType) -> Result<NativeValue> {
        unsafe {
            Ok(match ty {
                NativeType::JsonArray => {
                    NativeValue::Array(Array::ref_from_dpi_ptr(&value.asJsonArray))
                }
                NativeType::JsonObject => {
                    NativeValue::Object(Object::ref_from_dpi_ptr(&value.asJsonObject))
                }
                NativeType::Bytes => NativeValue::Bytes(slice::from_raw_parts(
                    value.asBytes.ptr as *mut u8,
                    value.asBytes.length as usize,
                )),
                NativeType::Float => NativeValue::Float(value.asFloat),
                NativeType::Double => NativeValue::Double(value.asDouble),
                NativeType::Timestamp => NativeValue::Timestamp(value.asTimestamp.into()),
                NativeType::IntervalDS => NativeValue::IntervalDS(value.asIntervalDS.into()),
                NativeType::IntervalYM => NativeValue::IntervalYM(value.asIntervalYM.into()),
                NativeType::Boolean => NativeValue::Boolean(value.asBoolean != 0),
                NativeType::Null => NativeValue::Null,
                _ => {
                    return Err(Error::other(format!("unexpected native type {:?}", ty)));
                }
            })
        }
    }
}
