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
use crate::Error;
use crate::Result;
use std::future::Future;

pub async fn spawn_blocking<F, R>(f: F) -> Result<R>
where
    F: FnOnce() -> Result<R> + Send + 'static,
    R: Send + 'static,
{
    match tokio::task::spawn_blocking(f).await {
        Ok(res) => res,
        Err(err) => Err(Error::other(format!("blocking task {}", err))),
    }
}

pub fn spawn<F>(f: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let _ = tokio::task::spawn(f);
}
