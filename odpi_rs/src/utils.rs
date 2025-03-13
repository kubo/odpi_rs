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
use crate::context::Context;
use crate::Error;
use crate::Result;
use odpic_sys::{dpiContext_freeStringList, dpiStringList};
use std::borrow::Cow;
use std::ffi::c_char;
use std::ffi::{c_int, CStr};
use std::fmt;
use std::ptr;
use std::slice;
use std::str;
use std::time::Duration;

pub(crate) trait ToDpi<T>: Sized {
    fn to_dpi(&self) -> T;
}

impl ToDpi<*const c_char> for Option<Cow<'_, CStr>> {
    fn to_dpi(&self) -> *const c_char {
        self.as_ref().map(|c| c.as_ptr()).unwrap_or(ptr::null())
    }
}

impl ToDpi<c_int> for bool {
    fn to_dpi(&self) -> c_int {
        (*self).into()
    }
}

impl ToDpi<*const c_char> for *const u8 {
    fn to_dpi(&self) -> *const c_char {
        *self as *const c_char
    }
}

impl ToDpi<(*const c_char, u32)> for &str {
    fn to_dpi(&self) -> (*const c_char, u32) {
        if self.is_empty() {
            (ptr::null(), 0)
        } else {
            (self.as_ptr() as *const c_char, self.len() as u32)
        }
    }
}

impl ToDpi<(*const c_char, u32)> for &[u8] {
    fn to_dpi(&self) -> (*const c_char, u32) {
        if self.is_empty() {
            (ptr::null(), 0)
        } else {
            (self.as_ptr() as *const c_char, self.len() as u32)
        }
    }
}

pub(crate) trait TryToDpi<T>: Sized {
    fn try_to_dpi(&self) -> Result<T>;
}

impl<T, U> TryToDpi<U> for T
where
    T: ToDpi<U>,
{
    fn try_to_dpi(&self) -> Result<U> {
        Ok(self.to_dpi())
    }
}

pub(crate) trait ToRust<T>: Sized {
    fn to_rust(&self) -> T;
}

impl ToRust<bool> for c_int {
    fn to_rust(&self) -> bool {
        *self != 0
    }
}

pub(crate) trait TryToRust<T>: Sized {
    fn try_to_rust(&self) -> Result<T>;
}

impl<'a> TryToRust<&'a [u8]> for (*const c_char, u32) {
    fn try_to_rust(&self) -> crate::Result<&'a [u8]> {
        Ok(unsafe { slice::from_raw_parts(self.0 as *mut u8, self.1.try_into()?) })
    }
}

impl<'a> TryToRust<&'a [u8]> for (*mut c_char, u32) {
    fn try_to_rust(&self) -> crate::Result<&'a [u8]> {
        Ok(unsafe { slice::from_raw_parts(self.0 as *mut u8, self.1.try_into()?) })
    }
}

impl<'a> TryToRust<Option<&'a [u8]>> for (*const c_char, u32) {
    fn try_to_rust(&self) -> crate::Result<Option<&'a [u8]>> {
        Ok(if self.0.is_null() {
            None
        } else {
            Some(self.try_to_rust()?)
        })
    }
}

impl TryToRust<Vec<u8>> for (*const c_char, u32) {
    fn try_to_rust(&self) -> crate::Result<Vec<u8>> {
        let slice: &[u8] = self.try_to_rust()?;
        Ok(slice.to_vec())
    }
}

impl TryToRust<Vec<u8>> for (*mut c_char, u32) {
    fn try_to_rust(&self) -> crate::Result<Vec<u8>> {
        let slice: &[u8] = self.try_to_rust()?;
        Ok(slice.to_vec())
    }
}

impl<'a> TryToRust<&'a str> for (*const c_char, u32) {
    fn try_to_rust(&self) -> crate::Result<&'a str> {
        let slice: &[u8] = self.try_to_rust()?;
        Ok(str::from_utf8(slice)?)
    }
}

impl<'a> TryToRust<Option<&'a str>> for (*const c_char, u32) {
    fn try_to_rust(&self) -> crate::Result<Option<&'a str>> {
        let opt_slice: Option<&[u8]> = self.try_to_rust()?;
        Ok(opt_slice.map(str::from_utf8).transpose()?)
    }
}

impl TryToRust<String> for (*const c_char, u32) {
    fn try_to_rust(&self) -> crate::Result<String> {
        let slice: &[u8] = self.try_to_rust()?;
        Ok(String::from_utf8_lossy(slice).into_owned())
    }
}

impl TryToRust<String> for (*mut c_char, u32) {
    fn try_to_rust(&self) -> crate::Result<String> {
        let slice: &[u8] = self.try_to_rust()?;
        Ok(String::from_utf8_lossy(slice).into_owned())
    }
}

impl TryToRust<Option<String>> for (*const c_char, u32) {
    fn try_to_rust(&self) -> crate::Result<Option<String>> {
        if self.0.is_null() {
            Ok(None)
        } else {
            Ok(Some(self.try_to_rust()?))
        }
    }
}

pub(crate) trait TryIntoRust<T>: Sized {
    fn try_into_rust(self) -> Result<T>;
}

impl TryIntoRust<Vec<String>> for dpiStringList {
    fn try_into_rust(mut self) -> Result<Vec<String>> {
        fn try_to_rust(list: &dpiStringList) -> Result<Vec<String>> {
            unsafe {
                let num = list.__bindgen_anon_1.numStrings.try_into()?;
                let strings = slice::from_raw_parts(list.__bindgen_anon_2.strings, num);
                let lengths = slice::from_raw_parts(list.__bindgen_anon_3.stringLengths, num);
                strings
                    .iter()
                    .zip(lengths)
                    .map(|(ptr, len)| (*ptr, *len).try_to_rust())
                    .collect()
            }
        }
        let ctxt = Context::get().unwrap();
        let result = try_to_rust(&self);
        unsafe {
            dpiContext_freeStringList(ctxt.handle, &mut self);
        }
        result
    }
}

pub(crate) trait ToPtr {
    fn to_ptr(&self) -> *mut c_char;
}

impl<T: AsRef<str>> ToPtr for T {
    fn to_ptr(&self) -> *mut c_char {
        self.as_ref().as_ptr() as *mut c_char
    }
}

pub(crate) trait TryToLen: Sized {
    fn try_to_len(&self) -> Result<u32>;
}

impl<T: AsRef<str>> TryToLen for T {
    fn try_to_len(&self) -> Result<u32> {
        Ok(self.as_ref().len().try_into()?)
    }
}

pub(crate) trait Num {
    fn is_zero(&self) -> bool;
    fn one() -> Self;
}

impl Num for i32 {
    fn is_zero(&self) -> bool {
        *self == 0
    }
    fn one() -> Self {
        1
    }
}

impl Num for u32 {
    fn is_zero(&self) -> bool {
        *self == 0
    }
    fn one() -> Self {
        1
    }
}

pub(crate) fn duration_to_secs<T>(value: Duration, msg: &str) -> Result<T>
where
    T: TryFrom<u64> + Num,
{
    let secs: T = value
        .as_secs()
        .try_into()
        .map_err(|_| Error::other(format!("too long duration {:?} for {}", value, msg)))?;
    Ok(if secs.is_zero() && !value.is_zero() {
        T::one() // when 0 < value < 1 second
    } else {
        secs
    })
}

pub(crate) fn duration_to_millis<T>(value: Duration, msg: &str) -> Result<T>
where
    T: TryFrom<u128> + Num,
{
    let msecs: T = value
        .as_millis()
        .try_into()
        .map_err(|_| Error::other(format!("too long duration {:?} for {}", value, msg)))?;
    Ok(if msecs.is_zero() && !value.is_zero() {
        T::one() // when 0 < value < 1 millisecond
    } else {
        msecs
    })
}

// struct just writing "..." by fmt::Debug
pub(crate) struct FmtEllipsis();

impl fmt::Debug for FmtEllipsis {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        f.write_str("...")
    }
}
