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
use crate::types::json::{array, object, NativeValue};
use crate::types::{IntervalDS, IntervalYM, NativeType, OracleType, Timestamp};
use crate::utils::*;
use crate::{Error, Result};
use odpic_sys::*;
use std::marker::PhantomData;
use std::ops::Index;
use std::slice;
use std::str;

pub struct Node<'a> {
    inner: dpiJsonNode,
    phantom: PhantomData<&'a ()>,
}

impl Node<'_> {
    pub fn oracle_type(&self) -> Result<OracleType> {
        self.inner.oracleTypeNum.try_to_rust()
    }

    pub fn data(&self) -> Result<NativeValue> {
        NativeValue::from_dpi(
            unsafe { &*self.inner.value },
            self.inner.nativeTypeNum.try_to_rust()?,
        )
    }

    pub fn to_owned(&self) -> Result<Value> {
        self.try_into()
    }

    pub(crate) unsafe fn ref_from_dpi_ptr<'a>(ptr: *const dpiJsonNode) -> &'a Node<'a> {
        &*(ptr as *const Node)
    }
}

impl ToDpi<*mut dpiJsonNode> for Node<'_> {
    fn to_dpi(&self) -> *mut dpiJsonNode {
        &self.inner as *const dpiJsonNode as *mut dpiJsonNode
    }
}

pub type Map<K, V> = std::collections::HashMap<K, V>;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Number {
    String(String),
    Double(f64),
    Float(f32),
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum DateTime {
    Timestamp(Timestamp),
    Double(f64),
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
    Raw(Vec<u8>),
    JsonId(Vec<u8>),
    Date(DateTime),
    Timestamp(DateTime),
    IntervalDS(IntervalDS),
    IntervalYM(IntervalYM),
    #[doc(hidden)]
    Vector(Vec<u8>),
}

impl Value {
    fn from_dpi(
        oracle_type: OracleType,
        native_type: NativeType,
        value: &dpiDataBuffer,
    ) -> Result<Value> {
        unsafe {
            match (oracle_type, native_type) {
                (OracleType::Varchar, NativeType::Bytes) => Ok(Value::String(
                    (value.asBytes.ptr, value.asBytes.length).try_to_rust()?,
                )),
                (OracleType::Raw, NativeType::Bytes) => Ok(Value::Raw(
                    (value.asBytes.ptr, value.asBytes.length).try_to_rust()?,
                )),
                (OracleType::JsonId, NativeType::Bytes) => Ok(Value::JsonId(
                    (value.asBytes.ptr, value.asBytes.length).try_to_rust()?,
                )),
                (OracleType::Number, NativeType::Float) => {
                    Ok(Value::Number(Number::Float(value.asFloat)))
                }
                (OracleType::Number, NativeType::Double) => {
                    Ok(Value::Number(Number::Double(value.asDouble)))
                }
                (OracleType::Number, NativeType::Bytes) => {
                    Ok(Value::Number(Number::String(String::from_utf8_unchecked(
                        (value.asBytes.ptr, value.asBytes.length).try_to_rust()?,
                    ))))
                }
                (OracleType::Date, NativeType::Double) => {
                    Ok(Value::Date(DateTime::Double(value.asDouble)))
                }
                (OracleType::Date, NativeType::Timestamp) => {
                    Ok(Value::Date(DateTime::Timestamp(value.asTimestamp.into())))
                }
                (OracleType::Timestamp, NativeType::Double) => {
                    Ok(Value::Timestamp(DateTime::Double(value.asDouble)))
                }
                (OracleType::Timestamp, NativeType::Timestamp) => Ok(Value::Timestamp(
                    DateTime::Timestamp(value.asTimestamp.into()),
                )),
                (OracleType::IntervalDS, NativeType::IntervalDS) => {
                    Ok(Value::IntervalDS(value.asIntervalDS.into()))
                }
                (OracleType::IntervalYM, NativeType::IntervalYM) => {
                    Ok(Value::IntervalYM(value.asIntervalYM.into()))
                }
                (OracleType::Boolean, NativeType::Boolean) => Ok(Value::Bool(value.asBoolean != 0)),
                (OracleType::None, NativeType::Null) => Ok(Value::Null),
                (OracleType::Vector, NativeType::Bytes) => Ok(Value::Vector(
                    (value.asBytes.ptr, value.asBytes.length).try_to_rust()?,
                )),
                _ => Err(Error::other(format!(
                    "unexpected Oracle type and native type pair: {:?} and {:?}",
                    oracle_type, native_type
                ))),
            }
        }
    }
}

impl<'a> TryFrom<&'a Node<'a>> for Value {
    type Error = Error;

    fn try_from(value: &'a Node) -> Result<Value> {
        Value::from_dpi(
            value.inner.oracleTypeNum.try_to_rust()?,
            value.inner.nativeTypeNum.try_to_rust()?,
            unsafe { &*value.inner.value },
        )
    }
}

pub struct Array<'a> {
    inner: dpiJsonArray,
    phantom: PhantomData<&'a ()>,
}

impl Array<'_> {
    pub(crate) unsafe fn ref_from_dpi_ptr<'a>(ptr: *const dpiJsonArray) -> &'a Array<'a> {
        &*(ptr as *const Array)
    }

    pub fn len(&self) -> usize {
        self.inner.numElements as usize
    }

    /// Returns a reference to an element
    pub fn get(&self, index: usize) -> Option<&Node> {
        if index >= self.len() {
            None
        } else {
            Some(unsafe { self.get_unchecked(index) })
        }
    }

    /// Returns a reference to an element, without doing bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior.
    pub unsafe fn get_unchecked(&self, index: usize) -> &Node {
        Node::ref_from_dpi_ptr(self.inner.elements.add(index))
    }

    pub fn iter(&self) -> array::Iter {
        array::Iter::new(self)
    }
}

/// Returns a reference to the value corresponding to the supplied index.
///
/// # Panics
///
/// Panics if index >= array length
impl<'a> Index<usize> for Array<'a> {
    type Output = Node<'a>;

    fn index(&self, index: usize) -> &Node<'a> {
        if index >= self.len() {
            panic!(
                "index {index} out of range for Json array of length {}",
                self.len()
            );
        }
        unsafe { Node::ref_from_dpi_ptr(self.inner.elements.add(index)) }
    }
}

pub struct Object<'a> {
    inner: dpiJsonObject,
    phantom: PhantomData<&'a ()>,
}

impl Object<'_> {
    pub(crate) unsafe fn ref_from_dpi_ptr<'a>(ptr: *const dpiJsonObject) -> &'a Object<'a> {
        &*(ptr as *const Object)
    }

    pub fn len(&self) -> usize {
        self.inner.numFields as usize
    }

    pub(crate) unsafe fn key_as_bytes(&self, index: usize) -> &[u8] {
        slice::from_raw_parts(
            *self.inner.fieldNames.add(index) as *const u8,
            *self.inner.fieldNameLengths.add(index) as usize,
        )
    }

    pub fn get<T>(&self, key: T) -> Option<&Node>
    where
        T: AsRef<str>,
    {
        let key = key.as_ref().as_bytes();
        unsafe {
            for index in 0..self.len() {
                if key == self.key_as_bytes(index) {
                    return Some(self.value_unchecked(index));
                }
            }
        }
        None
    }

    /// Returns a reference to an key.
    pub fn key(&self, index: usize) -> Option<&str> {
        if index < self.len() {
            str::from_utf8(unsafe { self.key_as_bytes(index) }).ok()
        } else {
            None
        }
    }

    /// Returns a reference to an key, without doing bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior.
    pub unsafe fn key_unchecked(&self, index: usize) -> &str {
        str::from_utf8_unchecked(self.key_as_bytes(index))
    }

    /// Returns a reference to an element.
    pub fn value(&self, index: usize) -> Option<&Node> {
        if index < self.len() {
            Some(unsafe { self.value_unchecked(index) })
        } else {
            None
        }
    }

    /// Returns a reference to an field value, without doing bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior.
    pub unsafe fn value_unchecked(&self, index: usize) -> &Node {
        Node::ref_from_dpi_ptr(self.inner.fields.add(index))
    }

    pub fn iter(&self) -> object::Iter {
        object::Iter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn check_size() {
        assert_eq!(size_of::<Node>(), size_of::<dpiJsonNode>());
        assert_eq!(size_of::<Array>(), size_of::<dpiJsonArray>());
        assert_eq!(size_of::<Object>(), size_of::<dpiJsonObject>());
    }
}
