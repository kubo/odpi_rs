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
pub struct IntervalDS {
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
    pub fseconds: i32,
}

impl From<IntervalDS> for dpiIntervalDS {
    fn from(value: IntervalDS) -> dpiIntervalDS {
        dpiIntervalDS {
            days: value.days,
            hours: value.hours,
            minutes: value.minutes,
            seconds: value.seconds,
            fseconds: value.fseconds,
        }
    }
}

impl From<dpiIntervalDS> for IntervalDS {
    fn from(value: dpiIntervalDS) -> IntervalDS {
        IntervalDS {
            days: value.days,
            hours: value.hours,
            minutes: value.minutes,
            seconds: value.seconds,
            fseconds: value.fseconds,
        }
    }
}
