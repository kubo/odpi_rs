use crate::types::Json;
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::ffi::CStr;

#[derive(Debug)]
#[odpic_doc(name = "dpiSodaDoc")]
pub struct Doc {
    pub(crate) handle: *mut dpiSodaDoc,
}

#[odpic_doc(name = "dpiSodaDoc")]
impl Doc {
    pub(crate) fn new(handle: *mut dpiSodaDoc) -> Doc {
        Doc { handle }
    }

    pub fn content(&self) -> Result<(Option<Vec<u8>>, Option<String>)> {
        let (content, encoding) = unsafe { self.content_as_slice()? };
        Ok((content.map(Vec::from), encoding.map(str::to_string)))
    }

    #[odpic_doc(name = "dpiSodaDoc_getContent")]
    pub unsafe fn content_as_slice(&self) -> Result<(Option<&[u8]>, Option<&str>)> {
        let (ptr, len, encoding) = get_3values!(dpiSodaDoc_getContent(self.handle))?;
        Ok((
            (ptr, len).try_to_rust()?,
            if encoding.is_null() {
                None
            } else {
                unsafe { CStr::from_ptr(encoding) }.to_str().ok()
            },
        ))
    }

    pub fn created_on(&self) -> Result<Option<String>> {
        Ok(unsafe { self.created_on_as_slice()? }.map(str::to_string))
    }

    #[odpic_doc(name = "dpiSodaDoc_getCreatedOn")]
    pub unsafe fn created_on_as_slice(&self) -> Result<Option<&str>> {
        let (ptr, len) = get_2values!(dpiSodaDoc_getCreatedOn(self.handle))?;
        (ptr, len).try_to_rust()
    }

    pub fn is_json(&self) -> Result<bool> {
        Ok(get_value!(dpiSodaDoc_getIsJson(self.handle))? != 0)
    }

    pub fn json_content(&self) -> Result<Json> {
        Ok(Json::new(get_value!(dpiSodaDoc_getJsonContent(
            self.handle
        ))?))
    }

    pub fn key(&self) -> Result<Option<String>> {
        Ok(unsafe { self.key_as_slice()? }.map(str::to_string))
    }

    #[odpic_doc(name = "dpiSodaDoc_getKey")]
    pub unsafe fn key_as_slice(&self) -> Result<Option<&str>> {
        let (ptr, len) = get_2values!(dpiSodaDoc_getKey(self.handle))?;
        (ptr, len).try_to_rust()
    }

    pub fn last_modified(&self) -> Result<Option<String>> {
        Ok(unsafe { self.last_modified_as_slice()? }.map(str::to_string))
    }

    #[odpic_doc(name = "dpiSodaDoc_getLastModified")]
    pub unsafe fn last_modified_as_slice(&self) -> Result<Option<&str>> {
        let (ptr, len) = get_2values!(dpiSodaDoc_getLastModified(self.handle))?;
        (ptr, len).try_to_rust()
    }

    pub fn media_type(&self) -> Result<Option<String>> {
        Ok(unsafe { self.media_type_as_slice()? }.map(str::to_string))
    }

    #[odpic_doc(name = "dpiSodaDoc_getMediaType")]
    pub unsafe fn media_type_as_slice(&self) -> Result<Option<&str>> {
        let (ptr, len) = get_2values!(dpiSodaDoc_getMediaType(self.handle))?;
        (ptr, len).try_to_rust()
    }

    pub fn version(&self) -> Result<Option<String>> {
        Ok(unsafe { self.version_as_slice()? }.map(str::to_string))
    }

    #[odpic_doc(name = "dpiSodaDoc_getVersion")]
    pub unsafe fn version_as_slice(&self) -> Result<Option<&str>> {
        let (ptr, len) = get_2values!(dpiSodaDoc_getVersion(self.handle))?;
        (ptr, len).try_to_rust()
    }
}

impl Clone for Doc {
    fn clone(&self) -> Doc {
        unsafe { dpiSodaDoc_addRef(self.handle) };
        Doc {
            handle: self.handle,
        }
    }
}

impl Drop for Doc {
    fn drop(&mut self) {
        release_handle!(dpiSodaDoc_release(self.handle));
    }
}
