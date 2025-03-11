use crate::context::Context;
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::boxed::Box;
use std::ffi::c_char;
use std::pin::Pin;
use std::ptr;
use std::slice;

#[derive(Clone, Debug)]
#[non_exhaustive]
#[odpic_doc(name = "dpiSodaOperOptions")]
pub struct OperOptions {
    pub keys: Vec<String>,
    pub key: String,
    pub version: String,
    pub filter: String,
    pub skip: u32,
    pub limit: u32,
    pub fetch_array_size: u32,
    pub hint: String,
    pub lock: bool,
}

impl OperOptions {
    /// Create `OperOptions`
    ///
    /// # Note
    /// This intanally calls [`Context::get()`].
    ///
    /// If you need to call [`Context::init()`], it must be called before this.
    pub fn new() -> Result<OperOptions> {
        let ctxt = Context::get()?;
        let opts = get_value!(dpiContext_initSodaOperOptions(ctxt.handle))?;
        let nkeys = opts.numKeys.try_into()?;
        let keys = unsafe { slice::from_raw_parts(opts.keys, nkeys) };
        let key_lengths = unsafe { slice::from_raw_parts(opts.keyLengths, nkeys) };
        Ok(OperOptions {
            keys: keys
                .iter()
                .zip(key_lengths)
                .map(|(key, len)| (*key, *len).try_to_rust())
                .collect::<Result<Vec<_>>>()?,
            key: (opts.key, opts.keyLength).try_to_rust()?,
            version: (opts.version, opts.versionLength).try_to_rust()?,
            filter: (opts.filter, opts.filterLength).try_to_rust()?,
            skip: opts.skip,
            limit: opts.limit,
            fetch_array_size: opts.fetchArraySize,
            hint: (opts.hint, opts.hintLength).try_to_rust()?,
            lock: opts.lock.to_rust(),
        })
    }

    /// Appends `keys` to [`field@OperOptions::keys`] field.
    pub fn append_keys<I>(&mut self, keys: I) -> &mut Self
    where
        I: IntoIterator<Item = String>,
    {
        self.keys.extend(keys);
        self
    }

    /// Sets `value` to [`field@OperOptions::key`] field.
    pub fn key<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.key = value.into();
        self
    }

    /// Sets `value` to [`field@OperOptions::version`] field.
    pub fn version<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.version = value.into();
        self
    }

    /// Sets `value` to [`field@OperOptions::filter`] field.
    pub fn filter<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.filter = value.into();
        self
    }

    /// Sets `value` to [`field@OperOptions::skip`] field.
    pub fn skip<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<u32>,
    {
        self.skip = value.into();
        self
    }

    /// Sets `value` to [`field@OperOptions::limit`] field.
    pub fn limit<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<u32>,
    {
        self.limit = value.into();
        self
    }

    /// Sets `value` to [`field@OperOptions::fetch_array_size`] field.
    pub fn fetch_array_size<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<u32>,
    {
        self.fetch_array_size = value.into();
        self
    }

    /// Sets `value` to [`field@OperOptions::hint`] field.
    pub fn hint<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.hint = value.into();
        self
    }

    /// Sets `value` to [`field@OperOptions::lock`] field.
    pub fn lock(&mut self, value: bool) -> &mut Self {
        self.lock = value;
        self
    }
}

#[derive(Default)]
pub(crate) struct DpiOperOptions {
    opts: Option<dpiSodaOperOptions>,
    _keys: Option<Pin<Box<[*const c_char]>>>,
    _lengths: Option<Pin<Box<[u32]>>>,
}

impl DpiOperOptions {
    pub(crate) fn as_ptr(&self) -> *const dpiSodaOperOptions {
        self.opts
            .as_ref()
            .map(|opts| opts as *const dpiSodaOperOptions)
            .unwrap_or(ptr::null())
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut dpiSodaOperOptions {
        self.opts
            .as_mut()
            .map(|opts| opts as *mut dpiSodaOperOptions)
            .unwrap_or(ptr::null_mut())
    }
}

impl TryToDpi<DpiOperOptions> for OperOptions {
    fn try_to_dpi(&self) -> Result<DpiOperOptions> {
        let (mut keys, mut lengths) = if !self.keys.is_empty() {
            let mut keys = Vec::with_capacity(self.keys.len());
            let mut lengths = Vec::with_capacity(self.keys.len());
            for key in &self.keys {
                keys.push(key.to_ptr() as *const c_char);
                lengths.push(key.try_to_len()?);
            }
            (
                Some(Box::into_pin(keys.into_boxed_slice())),
                Some(Box::into_pin(lengths.into_boxed_slice())),
            )
        } else {
            (None, None)
        };
        Ok(DpiOperOptions {
            opts: Some(dpiSodaOperOptions {
                numKeys: self.keys.len() as u32,
                keys: keys
                    .as_mut()
                    .map(|key| key.as_mut_ptr())
                    .unwrap_or(ptr::null_mut()),
                keyLengths: lengths
                    .as_mut()
                    .map(|len| len.as_mut_ptr())
                    .unwrap_or(ptr::null_mut()),
                key: self.key.to_ptr(),
                keyLength: self.key.try_to_len()?,
                version: self.version.to_ptr(),
                versionLength: self.version.try_to_len()?,
                filter: self.filter.to_ptr(),
                filterLength: self.filter.try_to_len()?,
                skip: self.skip,
                limit: self.limit,
                fetchArraySize: self.fetch_array_size,
                hint: self.hint.to_ptr(),
                hintLength: self.hint.try_to_len()?,
                lock: self.lock.to_dpi(),
            }),
            _keys: keys,
            _lengths: lengths,
        })
    }
}

impl TryToDpi<DpiOperOptions> for Option<&OperOptions> {
    fn try_to_dpi(&self) -> Result<DpiOperOptions> {
        if let Some(opts) = self {
            opts.try_to_dpi()
        } else {
            Ok(Default::default())
        }
    }
}
