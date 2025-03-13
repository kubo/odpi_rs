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
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

dpi_bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc(name = "dpiSodaFlags")]
    pub struct Flags : u32 {
        #[odpic_doc]
        const ATOMIC_COMMIT = DPI_SODA_FLAGS_ATOMIC_COMMIT;
        #[odpic_doc]
        const CREATE_COLL_MAP = DPI_SODA_FLAGS_CREATE_COLL_MAP;
        #[odpic_doc]
        const DEFAULT = DPI_SODA_FLAGS_DEFAULT;
        #[odpic_doc]
        const INDEX_DROP_FORCE = DPI_SODA_FLAGS_INDEX_DROP_FORCE;
    }
}
