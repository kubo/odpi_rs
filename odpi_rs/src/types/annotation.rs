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
use crate::utils::*;
use crate::*;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug, Clone)]
#[odpic_doc]
pub struct Annotation {
    pub key: String,
    pub value: String,
}

impl Annotation {
    pub(crate) fn from_dpi(annotation: &dpiAnnotation) -> Result<Annotation> {
        Ok(Annotation {
            key: (annotation.key, annotation.keyLength).try_to_rust()?,
            value: (annotation.value, annotation.valueLength).try_to_rust()?,
        })
    }
}
