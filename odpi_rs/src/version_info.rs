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
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VersionInfo {
    pub version_num: i32,
    pub release_num: i32,
    pub update_num: i32,
    pub port_release_num: i32,
    pub port_update_num: i32,
}

impl VersionInfo {
    pub fn new(
        version_num: i32,
        release_num: i32,
        update_num: i32,
        port_release_num: i32,
        port_update_num: i32,
    ) -> VersionInfo {
        VersionInfo {
            version_num,
            release_num,
            update_num,
            port_release_num,
            port_update_num,
        }
    }
}

impl From<dpiVersionInfo> for VersionInfo {
    fn from(ver: dpiVersionInfo) -> VersionInfo {
        VersionInfo {
            version_num: ver.versionNum,
            release_num: ver.releaseNum,
            update_num: ver.updateNum,
            port_release_num: ver.portReleaseNum,
            port_update_num: ver.portUpdateNum,
        }
    }
}

impl From<VersionInfo> for dpiVersionInfo {
    fn from(ver: VersionInfo) -> dpiVersionInfo {
        dpiVersionInfo {
            versionNum: ver.version_num,
            releaseNum: ver.release_num,
            updateNum: ver.update_num,
            portReleaseNum: ver.port_release_num,
            portUpdateNum: ver.port_update_num,
            fullVersionNum: (ver.version_num * 100000000
                + ver.release_num * 1000000
                + ver.update_num * 10000
                + ver.port_release_num * 100
                + ver.port_update_num) as u32,
        }
    }
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}.{}.{}.{}.{}",
            self.version_num,
            self.release_num,
            self.update_num,
            self.port_release_num,
            self.port_update_num
        )
    }
}
