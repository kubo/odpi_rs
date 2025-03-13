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
use crate::aq::{DeqOptions, EnqOptions, MsgProps};
use crate::maybe_async;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug)]
#[odpic_doc]
pub struct Queue {
    pub(crate) handle: *mut dpiQueue,
}

#[odpic_doc]
impl Queue {
    pub(crate) fn new(handle: *mut dpiQueue) -> Queue {
        Queue { handle }
    }

    #[maybe_async]
    pub async fn deq_many(&self) -> Result<Vec<MsgProps>> {
        let (num, props) = get_2values_blocking! {
            let handle = self.handle;
            dpiQueue_deqMany(*handle)
        }
        .await?;
        Ok((0..(*num as usize))
            .map(|idx| MsgProps::new(unsafe { (*props).add(idx) }))
            .collect())
    }

    #[maybe_async]
    pub async fn deq_one(&self) -> Result<Option<MsgProps>> {
        let prop = get_value_blocking! {
            let handle = self.handle;
            dpiQueue_deqOne(*handle)
        }
        .await?;
        if prop.is_null() {
            Ok(None)
        } else {
            Ok(Some(MsgProps::new(*prop)))
        }
    }

    #[maybe_async]
    pub async fn enq_many<'a, T>(&self, props: T) -> Result<()>
    where
        T: IntoIterator<Item = &'a MsgProps>,
    {
        let mut props = props
            .into_iter()
            .map(|props| props.handle)
            .collect::<Vec<_>>();
        call_blocking! {
            let handle = self.handle;
            let props_len = props.len().try_into()?;
            let props_ptr = props.as_mut_ptr();
            dpiQueue_enqMany(
                *handle,
                *props_len,
                *props_ptr,
            )
        }
        .await
    }

    #[maybe_async]
    pub async fn enq_one(&self, props: &MsgProps) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            let props_handle = props.handle;
            dpiQueue_enqOne(*handle, *props_handle)
        }
        .await
    }

    pub fn deq_options(&self) -> Result<DeqOptions> {
        Ok(DeqOptions::new(get_value!(dpiQueue_getDeqOptions(
            self.handle
        ))?))
    }

    pub fn enq_options(&self) -> Result<EnqOptions> {
        Ok(EnqOptions::new(get_value!(dpiQueue_getEnqOptions(
            self.handle
        ))?))
    }
}

impl Clone for Queue {
    fn clone(&self) -> Queue {
        unsafe { dpiQueue_addRef(self.handle) };
        Queue {
            handle: self.handle,
        }
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        release_handle!(dpiQueue_release(self.handle));
    }
}
