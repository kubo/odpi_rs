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
#![doc = include_str!("../README.md")]
pub use odpi_rs_procmacro::{async_impl, main, maybe_async, sync_impl, test};
use std::ffi::c_char;
use std::ops::{Deref, DerefMut};
use std::slice;
use std::str;

// Re-exports
pub use odpic_sys;

macro_rules! call {
    ($c_func:ident($($arg:expr),+ $(,)?)) => {
        call!($c_func($($arg),+) -> Ok(()))
    };

    ($c_func:ident($($arg:expr),+) -> $ret:expr) => {{
        #[allow(unused_unsafe)]
        if unsafe { $c_func($($arg),+) } == 0 {
            $ret
        } else {
            Err($crate::context::Context::get()?.last_error())
        }
    }};
}

macro_rules! get_value {
    ($c_func:ident($($arg:expr),+ $(,)?)) => {{
        let mut __value = unsafe { ::std::mem::zeroed() };
        call!($c_func($($arg),+, &mut __value) -> Ok(__value))
    }};
}

macro_rules! get_2values {
    ($c_func:ident($($arg:expr),+ $(,)?)) => {{
        let mut __value1 = unsafe { ::std::mem::zeroed() };
        let mut __value2 = unsafe { ::std::mem::zeroed() };
        call!(
            $c_func($($arg),+, &mut __value1, &mut __value2)
            -> Ok((__value1, __value2))
        )
    }};
}

macro_rules! get_3values {
    ($c_func:ident($($arg:expr),+ $(,)?)) => {{
        let mut __value1 = unsafe { ::std::mem::zeroed() };
        let mut __value2 = unsafe { ::std::mem::zeroed() };
        let mut __value3 = unsafe { ::std::mem::zeroed() };
        call!(
            $c_func($($arg),+, &mut __value1, &mut __value2, &mut __value3)
            -> Ok((__value1, __value2, __value3))
        )
    }};
}

macro_rules! get_4values {
    ($c_func:ident($($arg:expr),+ $(,)?)) => {{
        let mut __value1 = unsafe { ::std::mem::zeroed() };
        let mut __value2 = unsafe { ::std::mem::zeroed() };
        let mut __value3 = unsafe { ::std::mem::zeroed() };
        let mut __value4 = unsafe { ::std::mem::zeroed() };
        call!(
            $c_func($($arg),+, &mut __value1, &mut __value2, &mut __value3, &mut __value4)
            -> Ok((__value1, __value2, __value3, __value4))
        )
    }};
}

macro_rules! get_string_value {
    ($c_func:ident($($arg:expr),+ $(,)?)) => {{
        let mut __ptr = ::std::ptr::null();
        let mut __len = 0;
        call!(
            $c_func($($arg),+, &mut __ptr, &mut __len)
            -> (__ptr, __len).try_to_rust()
        )
    }};
}

macro_rules! blocking {
    (let mut $var:ident $(: $ty:ty)? = $val:expr; $($tt:tt)+) => {{
        let mut $var $(: $crate::ForceSend<$ty>)? = $crate::ForceSend($val);
        blocking!{$($tt)+}
    }};

    (let $var:ident $(: $ty:ty)? = $val:expr; $($tt:tt)+) => {{
        let $var $(: $crate::ForceSend<$ty>)? = $crate::ForceSend($val);
        blocking!{$($tt)+}
    }};

    ($c_func:ident($($arg:expr),+ $(,)?) => $ret:expr $(=> $($out_arg:ident),+)? ) => {{
        $crate::task::spawn_blocking(move || -> $crate::Result<_> {
            use odpic_sys::blocking::$c_func;
            $($(let mut $out_arg = unsafe { ::std::mem::zeroed() }; )+)?
            if unsafe { $c_func($($arg),+, $($(&mut $out_arg),+)?) } == 0 {
                $ret
            } else {
                Err($crate::context::Context::get()?.last_error())
            }
        })
    }};
}

macro_rules! call_blocking {
    ($($tt:tt)+) => {
        blocking! { $($tt)+ => Ok(()) }
    };
}

macro_rules! get_value_blocking {
    ($($tt:tt)+) => {
        blocking! {
            $($tt)+
            => Ok($crate::ForceSend(__value))
            => __value
        }
    };
}

macro_rules! get_2values_blocking {
    ($($tt:tt)+) => {
        blocking! {
            $($tt)+
            => Ok(($crate::ForceSend(__value1), $crate::ForceSend(__value2)))
            => __value1, __value2
        }
    };
}

macro_rules! get_3values_blocking {
    ($($tt:tt)+) => {
        blocking! {
            $($tt)+
            => Ok(($crate::ForceSend(__value1), $crate::ForceSend(__value2), $crate::ForceSend(__value3)))
            => __value1, __value2, __value3
        }
    };
}

#[cfg(not(feature = "is_async"))]
macro_rules! release_handle {
    ($c_func:ident($handle:expr)) => {{
        use odpic_sys::blocking::$c_func;
        unsafe { $c_func($handle) };
    }};
}

#[cfg(feature = "is_async")]
macro_rules! release_handle {
    ($c_func:ident($handle:expr)) => {{
        let handle = $crate::ForceSend($handle);
        $crate::task::spawn(async move {
            use odpic_sys::blocking::$c_func;
            unsafe { $c_func(*handle) };
        });
    }};
}

macro_rules! set_str_value {
    ($c_func:ident($expr:expr, $value:expr $(,)?)) => {{
        use crate::utils::ToPtr;
        use crate::utils::TryToLen;
        call!($c_func($expr, $value.to_ptr(), $value.try_to_len()?))
    }};
}

macro_rules! dpi_enum {
    (
         $(#[$enum_attr:meta])* $vis:vis enum $enum_name:ident : $underlaying_type:ty {
            $(
               $(#[$variant_attr:meta])* $variant_name:ident = $variant_value:ident,
            )*
         }
    ) => {
        $(#[$enum_attr])* $vis enum $enum_name {
            $(
                $(#[$variant_attr])* $variant_name = $variant_value,
            )*
        }

        impl $crate::utils::ToDpi<$underlaying_type> for $enum_name {
            fn to_dpi(&self) -> $underlaying_type {
                *self as $underlaying_type
            }
        }

        impl $crate::utils::TryToRust<$enum_name> for $underlaying_type {
            fn try_to_rust(&self) -> $crate::Result<$enum_name> {
                match *self {
                    $(
                        $variant_value => Ok($enum_name :: $variant_name),
                    )*
                    _ => Err($crate::Error::other(format!(concat!("unexpected ", stringify!($underlaying_type), " {}"), self))),
                }
            }
        }
    };
}

macro_rules! dpi_bitflags {
    (
         $(#[$struct_attr:meta])* $vis:vis struct $struct_name:ident : $underlaying_type:ty {
            $(
               $(#[$const_attr:ident $($args:tt)*])* const $const_name:ident = $const_value:expr;
            )*
         }
    ) => {
        ::bitflags::bitflags! {
            $(#[$struct_attr])* $vis struct $struct_name : $underlaying_type {
                $(
                   $(#[$const_attr $($args)*])* const $const_name = $const_value;
                )*
            }
        }

        impl $crate::utils::ToDpi<$underlaying_type> for $struct_name {
            fn to_dpi(&self) -> $underlaying_type {
                self.bits()
            }
        }

        impl $crate::utils::ToRust<$struct_name> for $underlaying_type {
            fn to_rust(&self) -> $struct_name {
                $struct_name::from_bits_retain(*self)
            }
        }
    };
}

#[cfg(doc)]
pub mod __docs__;

pub mod aq;
pub mod conn;
pub mod context;
mod enums;
mod error;
pub mod soda;
pub mod stmt;
pub mod subscr;
#[cfg(not(doc))]
#[cfg_attr(not(feature = "is_async"), path = "task/is_sync.rs")]
#[cfg_attr(feature = "tokio", path = "task/tokio.rs")]
#[cfg_attr(feature = "async-std", path = "task/async_std.rs")]
#[cfg_attr(feature = "smol", path = "task/smol.rs")]
mod task;
pub mod types;
pub(crate) mod utils;
mod version_info;

pub use enums::*;
pub use error::Error;
pub use error::OdpiError;
pub use version_info::VersionInfo;

pub type Result<T> = std::result::Result<T, Error>;

#[allow(unused)]
trait AssertSend: Send {}
#[allow(unused)]
trait AssertSync: Sync {}

struct ForceSend<T>(T);

unsafe impl<T> Send for ForceSend<T> {}

impl<T> Deref for ForceSend<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ForceSend<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for bool {}
    impl Sealed for i8 {}
    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
    impl Sealed for &str {}
    impl Sealed for crate::types::Timestamp {}
    impl Sealed for crate::types::IntervalDS {}
    impl Sealed for crate::types::IntervalYM {}
    impl Sealed for crate::types::Object {}
    impl Sealed for crate::types::Rowid {}
    impl<T: Sealed> Sealed for Option<T> {}
}

#[allow(dead_code)]
#[doc(hidden)]
// #[cfg(doctest)] isn't usable here. See: https://github.com/rust-lang/rust/issues/67295
pub mod test_util {
    use crate::conn::Conn;
    use crate::context::Context;
    use crate::maybe_async;
    use crate::{Result, VersionInfo};
    use std::env;

    fn env_var_or(env_name: &str, default: &str) -> String {
        match env::var_os(env_name) {
            Some(env_var) => env_var.into_string().unwrap(),
            None => String::from(default),
        }
    }

    pub fn main_user() -> String {
        env_var_or("ODPIC_TEST_MAIN_USER", "odpic")
    }

    pub fn main_password() -> String {
        env_var_or("ODPIC_TEST_MAIN_PASSWORD", "welcome")
    }

    pub fn edition_user() -> String {
        env_var_or("ODPIC_TEST_EDITION_USER", "odpic_edition")
    }

    pub fn edition_password() -> String {
        env_var_or("ODPIC_TEST_EDITION_PASSWORD", "welcome")
    }

    pub fn connect_string() -> String {
        env_var_or("ODPIC_TEST_CONNECT_STRING", "localhost/orclpdb")
    }

    #[maybe_async]
    pub async fn connect() -> Result<Conn> {
        Conn::create(main_user(), main_password(), connect_string(), None, None).await
    }

    #[maybe_async]
    pub async fn check_version(
        conn: &Conn,
        client_ver: &VersionInfo,
        server_ver: &VersionInfo,
    ) -> Result<bool> {
        Ok(&Context::get()?.client_version()? >= client_ver
            && &conn.server_version().await?.1 >= server_ver)
    }
}
