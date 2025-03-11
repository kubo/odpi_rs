use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::borrow::Cow;

#[derive(Debug, Clone)]
#[odpic_doc]
pub struct MsgRecipient<'a> {
    pub name: Cow<'a, str>,
}

impl MsgRecipient<'_> {
    pub fn new<'a, T>(name: T) -> MsgRecipient<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        MsgRecipient { name: name.into() }
    }
}

impl TryToDpi<dpiMsgRecipient> for MsgRecipient<'_> {
    fn try_to_dpi(&self) -> Result<dpiMsgRecipient> {
        Ok(dpiMsgRecipient {
            name: self.name.to_ptr(),
            nameLength: self.name.try_to_len()?,
        })
    }
}
