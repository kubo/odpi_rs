use crate::maybe_async;
use crate::types::OracleType;
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::ffi::c_char;

#[derive(Debug)]
#[odpic_doc]
pub struct Lob {
    pub(crate) handle: *mut dpiLob,
}

impl Lob {
    pub(crate) fn new(handle: *mut dpiLob) -> Lob {
        Lob { handle }
    }

    pub(crate) fn with_add_ref(handle: *mut dpiLob) -> Lob {
        unsafe { dpiLob_addRef(handle) };
        Lob { handle }
    }

    #[maybe_async]
    pub async fn close(&self) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            dpiLob_close(*handle)
        }
        .await
    }

    #[maybe_async]
    pub async fn close_resource(&self) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            dpiLob_closeResource(*handle)
        }
        .await
    }

    #[maybe_async]
    pub async fn copy(&self) -> Result<Lob> {
        Ok(Lob::new(
            *get_value_blocking! {
                let handle = self.handle;
                dpiLob_copy(*handle)
            }
            .await?,
        ))
    }

    pub fn buffer_size(&self, size_in_chars: u64) -> Result<u64> {
        get_value!(dpiLob_getBufferSize(self.handle, size_in_chars))
    }

    #[maybe_async]
    pub async fn chunk_size(&self) -> Result<u32> {
        Ok(*get_value_blocking! {
            let handle = self.handle;
            dpiLob_getChunkSize(*handle)
        }
        .await?)
    }

    pub fn directory_and_file_name(&self) -> Result<(String, String)> {
        let (dname, dname_len, fname, fname_len) =
            get_4values!(dpiLob_getDirectoryAndFileName(self.handle))?;
        Ok((
            (dname, dname_len).try_to_rust()?,
            (fname, fname_len).try_to_rust()?,
        ))
    }

    #[maybe_async]
    pub async fn file_exists(&self) -> Result<bool> {
        Ok(*get_value_blocking! {
            let handle = self.handle;
            dpiLob_getFileExists(*handle)
        }
        .await?
            != 0)
    }

    #[maybe_async]
    pub async fn is_resource_open(&self) -> Result<bool> {
        Ok(*get_value_blocking! {
            let handle = self.handle;
            dpiLob_getIsResourceOpen(*handle)
        }
        .await?
            != 0)
    }

    #[maybe_async]
    pub async fn size(&self) -> Result<u64> {
        Ok(*get_value_blocking! {
            let handle = self.handle;
            dpiLob_getSize(*handle)
        }
        .await?)
    }

    pub fn get_type(&self) -> Result<OracleType> {
        get_value!(dpiLob_getType(self.handle))?.try_to_rust()
    }

    #[maybe_async]
    pub async fn open_resource(&self) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            dpiLob_openResource(*handle)
        }
        .await
    }

    #[maybe_async]
    pub async fn read_bytes(&self, offset: u64, amount: u64, buf: &mut [u8]) -> Result<usize> {
        let mut len = buf.len().try_into()?;
        let len = blocking! {
            let handle = self.handle;
            let buf_mut_ptr = buf.as_mut_ptr() as *mut c_char;
            dpiLob_readBytes(
                *handle,
                offset,
                amount,
                *buf_mut_ptr,
                &mut len,
            )
            => Ok(len)
        }
        .await?;
        Ok(len as usize)
    }

    pub fn set_directory_and_file_name<D, F>(&self, directory_alias: D, file_name: F) -> Result<()>
    where
        D: AsRef<str>,
        F: AsRef<str>,
    {
        call!(dpiLob_setDirectoryAndFileName(
            self.handle,
            directory_alias.to_ptr(),
            directory_alias.try_to_len()?,
            file_name.to_ptr(),
            file_name.try_to_len()?
        ))
    }

    #[maybe_async]
    pub async fn set_from_bytes<T>(&self, value: T) -> Result<()>
    where
        T: AsRef<[u8]>,
    {
        let value = value.as_ref();
        call_blocking! {
            let handle = self.handle;
            let value_ptr = value.as_ptr() as *const c_char;
            let value_len = value.len().try_into()?;
            dpiLob_setFromBytes(
                *handle,
                *value_ptr,
                *value_len,
            )
        }
        .await
    }

    #[maybe_async]
    pub async fn trim(&self, new_size: u64) -> Result<()> {
        call_blocking! {
            let handle = self.handle;
            dpiLob_trim(*handle, new_size)
        }
        .await
    }

    #[maybe_async]
    pub async fn write_bytes<T>(&self, offset: u64, value: T) -> Result<()>
    where
        T: AsRef<[u8]>,
    {
        let value = value.as_ref();
        call_blocking! {
            let handle = self.handle;
            let value_ptr = value.as_ptr() as *const c_char;
            let value_len = value.len().try_into()?;
            dpiLob_writeBytes(
                *handle,
                offset,
                *value_ptr,
                *value_len,
            )
        }
        .await
    }
}

impl Clone for Lob {
    fn clone(&self) -> Lob {
        unsafe { dpiLob_addRef(self.handle) };
        Lob {
            handle: self.handle,
        }
    }
}

impl Drop for Lob {
    fn drop(&mut self) {
        release_handle!(dpiLob_release(self.handle));
    }
}
