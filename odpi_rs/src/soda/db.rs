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
use crate::maybe_async;
use crate::soda::{Coll, CollCursor, Doc, Flags};
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug)]
#[odpic_doc(name = "dpiSodaDb")]
pub struct Db {
    pub(crate) handle: *mut dpiSodaDb,
}

#[odpic_doc(name = "dpiSodaDb")]
impl Db {
    pub(crate) fn new(handle: *mut dpiSodaDb) -> Db {
        Db { handle }
    }

    #[maybe_async]
    pub async fn create_collection<N, M>(&self, name: N, metadata: M, flags: Flags) -> Result<Coll>
    where
        N: AsRef<str>,
        M: AsRef<str>,
    {
        Ok(Coll::new(
            *get_value_blocking! {
                let handle = self.handle;
                let name_ptr = name.to_ptr();
                let name_len = name.try_to_len()?;
                let metadata_ptr = metadata.to_ptr();
                let metadata_len = metadata.try_to_len()?;
                dpiSodaDb_createCollection(
                    *handle,
                    *name_ptr,
                    *name_len,
                    *metadata_ptr,
                    *metadata_len,
                    flags.to_dpi()
                )
            }
            .await?,
        ))
    }

    pub fn create_document<K, C, M>(
        &self,
        key: K,
        content: C,
        metadata: M,
        flags: Flags,
    ) -> Result<Doc>
    where
        K: AsRef<str>,
        C: AsRef<[u8]>,
        M: AsRef<str>,
    {
        let (content, content_len) = content.as_ref().to_dpi();
        Ok(Doc::new(get_value!(dpiSodaDb_createDocument(
            self.handle,
            key.to_ptr(),
            key.try_to_len()?,
            content,
            content_len,
            metadata.to_ptr(),
            metadata.try_to_len()?,
            flags.to_dpi()
        ))?))
    }

    // // create a new SODA document with JSON content
    // DPI_EXPORT int dpiSodaDb_createJsonDocument(dpiSodaDb *db, const char *key,
    //         uint32_t keyLength, const dpiJsonNode *content, uint32_t flags,
    //         dpiSodaDoc **doc);

    #[maybe_async]
    pub async fn collections<T>(&self, start_name: T, flags: Flags) -> Result<CollCursor>
    where
        T: AsRef<str>,
    {
        Ok(CollCursor::new(
            *get_value_blocking! {
                let handle = self.handle;
                let start_name_ptr = start_name.to_ptr();
                let start_name_len = start_name.try_to_len()?;
                dpiSodaDb_getCollections(
                    *handle,
                    *start_name_ptr,
                    *start_name_len,
                    flags.to_dpi()
                )
            }
            .await?,
        ))
    }

    #[maybe_async]
    pub async fn collection_names<T>(
        &self,
        start_name: T,
        limit: u32,
        flags: Flags,
    ) -> Result<Vec<String>>
    where
        T: AsRef<str>,
    {
        get_value_blocking! {
            let handle = self.handle;
            let start_name_ptr = start_name.to_ptr();
            let start_name_len = start_name.try_to_len()?;
            dpiSodaDb_getCollectionNames(
                *handle,
                *start_name_ptr,
                *start_name_len,
                limit,
                flags.to_dpi()
            )
        }
        .await?
        .try_into_rust()
    }

    #[maybe_async]
    pub async fn open_collection<T>(&self, name: T, flags: Flags) -> Result<Coll>
    where
        T: AsRef<str>,
    {
        Ok(Coll::new(
            *get_value_blocking! {
                let handle = self.handle;
                let name_ptr = name.to_ptr();
                let name_len = name.try_to_len()?;
                dpiSodaDb_openCollection(
                    *handle,
                    *name_ptr,
                    *name_len,
                    flags.to_dpi()
                )
            }
            .await?,
        ))
    }
}

impl Clone for Db {
    fn clone(&self) -> Db {
        unsafe { dpiSodaDb_addRef(self.handle) };
        Db {
            handle: self.handle,
        }
    }
}

impl Drop for Db {
    fn drop(&mut self) {
        release_handle!(dpiSodaDb_release(self.handle));
    }
}
