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
