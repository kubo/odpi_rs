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
use odpic_sys::*;

#[derive(Clone, Copy, Debug)]
pub struct Timestamp {
    pub year: i16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub fsecond: u32,
    pub tz_hour_offset: i8,
    pub tz_minute_offset: i8,
}

impl From<Timestamp> for dpiTimestamp {
    fn from(value: Timestamp) -> dpiTimestamp {
        dpiTimestamp {
            year: value.year,
            month: value.month,
            day: value.day,
            hour: value.hour,
            minute: value.minute,
            second: value.second,
            fsecond: value.fsecond,
            tzHourOffset: value.tz_hour_offset,
            tzMinuteOffset: value.tz_minute_offset,
        }
    }
}

impl From<dpiTimestamp> for Timestamp {
    fn from(value: dpiTimestamp) -> Timestamp {
        Timestamp {
            year: value.year,
            month: value.month,
            day: value.day,
            hour: value.hour,
            minute: value.minute,
            second: value.second,
            fsecond: value.fsecond,
            tz_hour_offset: value.tzHourOffset,
            tz_minute_offset: value.tzMinuteOffset,
        }
    }
}
