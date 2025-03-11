use crate::maybe_async;
use crate::soda::{oper_options::DpiOperOptions, Doc, DocCursor, Flags, OperOptions};
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::blocking::*;
use odpic_sys::*;
use std::ptr;

#[derive(Debug)]
#[odpic_doc(name = "dpiSodaColl")]
pub struct Coll {
    pub(crate) handle: *mut dpiSodaColl,
}

#[odpic_doc(name = "dpiSodaColl")]
impl Coll {
    pub(crate) fn new(handle: *mut dpiSodaColl) -> Coll {
        Coll { handle }
    }

    #[maybe_async]
    pub async fn create_index<T>(&self, index_spec: T, flags: Flags) -> Result<()>
    where
        T: AsRef<str>,
    {
        call_blocking! {
            let handle = self.handle;
            let index_spec_ptr = index_spec.to_ptr();
            let index_spec_len = index_spec.try_to_len()?;
            dpiSodaColl_createIndex(
                *handle,
                *index_spec_ptr,
                *index_spec_len,
                flags.to_dpi()
            )
        }
        .await
    }

    #[maybe_async]
    pub async fn drop(&self, flags: Flags) -> Result<bool> {
        Ok(*get_value_blocking! {
            let handle = self.handle;
            dpiSodaColl_drop(*handle, flags.to_dpi())
        }
        .await?
            != 0)
    }

    #[maybe_async]
    pub async fn drop_index<T>(&self, name: T, flags: Flags) -> Result<bool>
    where
        T: AsRef<str>,
    {
        Ok(*get_value_blocking! {
            let handle = self.handle;
            let name_ptr = name.to_ptr();
            let name_len = name.try_to_len()?;
            dpiSodaColl_dropIndex(
                *handle,
                *name_ptr,
                *name_len,
                flags.to_dpi()
            )
        }
        .await?
            != 0)
    }

    #[maybe_async]
    pub async fn find(&self, options: &OperOptions, flags: Flags) -> Result<DocCursor> {
        Ok(DocCursor::new(
            *get_value_blocking! {
                let handle = self.handle;
                let opts = options.try_to_dpi()?;
                dpiSodaColl_find(
                    *handle,
                    opts.as_ptr(),
                    flags.to_dpi()
                )
            }
            .await?,
        ))
    }

    #[maybe_async]
    pub async fn find_one(&self, options: &OperOptions, flags: Flags) -> Result<Doc> {
        Ok(Doc::new(
            *get_value_blocking! {
                let handle = self.handle;
                let opts = options.try_to_dpi()?;
                dpiSodaColl_findOne(
                    *handle,
                    opts.as_ptr(),
                    flags.to_dpi()
                )
            }
            .await?,
        ))
    }

    #[maybe_async]
    pub async fn data_guide(&self, flags: Flags) -> Result<Doc> {
        Ok(Doc::new(
            *get_value_blocking! {
                let handle = self.handle;
                dpiSodaColl_getDataGuide(*handle, flags.to_dpi())
            }
            .await?,
        ))
    }

    #[maybe_async]
    pub async fn doc_count(&self, options: &OperOptions, flags: Flags) -> Result<u64> {
        Ok(*get_value_blocking! {
            let handle = self.handle;
            let opts = options.try_to_dpi()?;
            dpiSodaColl_getDocCount(
                *handle,
                opts.as_ptr(),
                flags.to_dpi()
            )
        }
        .await?)
    }

    pub fn metadata(&self) -> Result<String> {
        get_string_value!(dpiSodaColl_getMetadata(self.handle))
    }

    pub fn name(&self) -> Result<String> {
        get_string_value!(dpiSodaColl_getName(self.handle))
    }

    #[maybe_async]
    #[odpic_doc(name = "dpiSodaColl_insertManyWithOptions")]
    pub async fn insert_many<'a, T>(
        &self,
        docs: T,
        options: Option<&OperOptions>,
        flags: Flags,
    ) -> Result<()>
    where
        T: IntoIterator<Item = &'a Doc>,
    {
        let mut docs = docs.into_iter().map(|doc| doc.handle).collect::<Vec<_>>();
        call_blocking! {
            let handle = self.handle;
            let docs_len = docs.len().try_into()?;
            let docs_mut_ptr = docs.as_mut_ptr();
            let mut opts = options.try_to_dpi()?;
            dpiSodaColl_insertManyWithOptions(
                *handle,
                *docs_len,
                *docs_mut_ptr,
                opts.as_mut_ptr(),
                flags.to_dpi(),
                ptr::null_mut()
            )
        }
        .await
    }

    #[maybe_async]
    #[odpic_doc(name = "dpiSodaColl_insertManyWithOptions")]
    pub async fn insert_many_and_get<'a, T>(
        &self,
        docs: T,
        options: Option<&OperOptions>,
        flags: Flags,
    ) -> Result<Vec<Doc>>
    where
        T: IntoIterator<Item = &'a Doc>,
    {
        let mut docs = docs.into_iter().map(|doc| doc.handle).collect::<Vec<_>>();
        let mut inserted_docs = vec![ptr::null_mut(); docs.len()];
        call_blocking! {
            let handle = self.handle;
            let docs_len = docs.len().try_into()?;
            let docs_mut_ptr = docs.as_mut_ptr();
            let mut opts = options.try_to_dpi()?;
            let inserted_docs_mut_ptr = inserted_docs.as_mut_ptr();
            dpiSodaColl_insertManyWithOptions(
                *handle,
                *docs_len,
                *docs_mut_ptr,
                opts.as_mut_ptr(),
                flags.to_dpi(),
                *inserted_docs_mut_ptr,
            )
        }
        .await?;
        Ok(inserted_docs.into_iter().map(Doc::new).collect())
    }

    #[odpic_doc(name = "dpiSodaColl_insertOneWithOptions")]
    pub fn insert_one(&self, doc: &Doc, options: Option<&OperOptions>, flags: Flags) -> Result<()> {
        let mut opts: DpiOperOptions = options.try_to_dpi()?;
        call!(dpiSodaColl_insertOneWithOptions(
            self.handle,
            doc.handle,
            opts.as_mut_ptr(),
            flags.to_dpi(),
            ptr::null_mut()
        ))
    }

    #[odpic_doc(name = "dpiSodaColl_insertOneWithOptions")]
    pub fn insert_one_and_get(
        &self,
        doc: &Doc,
        options: Option<&OperOptions>,
        flags: Flags,
    ) -> Result<Doc> {
        let mut opts: DpiOperOptions = options.try_to_dpi()?;
        Ok(Doc::new(get_value!(dpiSodaColl_insertOneWithOptions(
            self.handle,
            doc.handle,
            opts.as_mut_ptr(),
            flags.to_dpi()
        ))?))
    }

    pub fn list_indexes(&self, flags: Flags) -> Result<Vec<String>> {
        get_value!(dpiSodaColl_listIndexes(self.handle, flags.to_dpi()))?.try_into_rust()
    }

    pub fn remove(&self, options: Option<&OperOptions>, flags: Flags) -> Result<u64> {
        let opts: DpiOperOptions = options.try_to_dpi()?;
        get_value!(dpiSodaColl_remove(
            self.handle,
            opts.as_ptr(),
            flags.to_dpi()
        ))
    }

    #[odpic_doc(name = "dpiSodaColl_replaceOne")]
    pub fn replace_one(
        &self,
        options: Option<&OperOptions>,
        doc: &Doc,
        flags: Flags,
    ) -> Result<bool> {
        let opts: DpiOperOptions = options.try_to_dpi()?;
        let mut replaced = 0;
        call!(dpiSodaColl_replaceOne(
            self.handle,
            opts.as_ptr(),
            doc.handle,
            flags.to_dpi(),
            &mut replaced,
            ptr::null_mut(),
        ))?;
        Ok(replaced != 0)
    }

    #[odpic_doc(name = "dpiSodaColl_replaceOne")]
    pub fn replace_one_and_get(
        &self,
        options: Option<&OperOptions>,
        doc: &Doc,
        flags: Flags,
    ) -> Result<(bool, Doc)> {
        let opts: DpiOperOptions = options.try_to_dpi()?;
        let (replaced, replaced_doc) = get_2values!(dpiSodaColl_replaceOne(
            self.handle,
            opts.as_ptr(),
            doc.handle,
            flags.to_dpi()
        ))?;
        Ok((replaced != 0, Doc::new(replaced_doc)))
    }

    #[maybe_async]
    #[odpic_doc(name = "dpiSodaColl_saveWithOptions")]
    pub async fn save(&self, doc: &Doc, options: Option<&OperOptions>, flags: Flags) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            let doc_handle = doc.handle;
            let mut opts = options.try_to_dpi()?;
            dpiSodaColl_saveWithOptions(
                *handle,
                *doc_handle,
                opts.as_mut_ptr(),
                flags.to_dpi(),
                ptr::null_mut(),
            )
        }
        .await
    }

    #[maybe_async]
    #[odpic_doc(name = "dpiSodaColl_saveWithOptions")]
    pub async fn save_and_get(
        &self,
        doc: &Doc,
        options: Option<&OperOptions>,
        flags: Flags,
    ) -> Result<Doc> {
        Ok(Doc::new(
            *get_value_blocking! {
                let handle = self.handle;
                let doc_handle = doc.handle;
                let mut opts = options.try_to_dpi()?;
                dpiSodaColl_saveWithOptions(
                    *handle,
                    *doc_handle,
                    opts.as_mut_ptr(),
                    flags.to_dpi()
                )
            }
            .await?,
        ))
    }

    #[maybe_async]
    pub async fn truncate(&self) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            dpiSodaColl_truncate(*handle)
        }
        .await
    }
}

impl Clone for Coll {
    fn clone(&self) -> Coll {
        unsafe { dpiSodaColl_addRef(self.handle) };
        Coll {
            handle: self.handle,
        }
    }
}

impl Drop for Coll {
    fn drop(&mut self) {
        release_handle!(dpiSodaColl_release(self.handle));
    }
}
