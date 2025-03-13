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
pub struct IntervalYM {
    pub years: i32,
    pub months: i32,
}

impl From<IntervalYM> for dpiIntervalYM {
    fn from(value: IntervalYM) -> dpiIntervalYM {
        dpiIntervalYM {
            years: value.years,
            months: value.months,
        }
    }
}

impl From<dpiIntervalYM> for IntervalYM {
    fn from(value: dpiIntervalYM) -> IntervalYM {
        IntervalYM {
            years: value.years,
            months: value.months,
        }
    }
}
