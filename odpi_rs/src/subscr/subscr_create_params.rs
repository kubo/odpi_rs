use crate::context::Context;
use crate::subscr::{
    GroupingClass, GroupingType, Message, Namespace, OpCode, Protocol, Qos, SubscrCallback,
    SubscrCallbackContext,
};
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::ptr;
use std::sync::Arc;
use std::time::Duration;

#[odpic_doc]
pub struct SubscrCreateParams {
    pub subscr_namespace: Namespace,
    pub protocol: Protocol,
    pub qos: Qos,
    pub operations: OpCode,
    pub port_number: u32,
    pub timeout: Duration,
    pub name: String,
    pub callback: Option<Arc<SubscrCallback>>,
    pub recipient_name: String,
    pub ip_address: String,
    pub grouping_class: GroupingClass,
    pub grouping_value: u32,
    pub grouping_type: GroupingType,
    pub out_reg_id: u64,
    pub client_initiated: bool,
}

impl SubscrCreateParams {
    pub fn new() -> Result<SubscrCreateParams> {
        let ctxt = Context::get()?;
        let params = get_value!(dpiContext_initSubscrCreateParams(ctxt.handle))?;
        Ok(SubscrCreateParams {
            subscr_namespace: params.subscrNamespace.try_to_rust()?,
            protocol: params.protocol.try_to_rust()?,
            qos: params.qos.to_rust(),
            operations: params.operations.to_rust(),
            port_number: params.portNumber,
            timeout: Duration::from_secs(params.timeout.into()),
            name: (params.name, params.nameLength).try_to_rust()?,
            callback: None,
            recipient_name: (params.recipientName, params.recipientNameLength).try_to_rust()?,
            ip_address: (params.ipAddress, params.ipAddressLength).try_to_rust()?,
            grouping_class: params.groupingClass.try_to_rust()?,
            grouping_value: params.groupingValue,
            grouping_type: params.groupingType.try_to_rust()?,
            out_reg_id: params.outRegId,
            client_initiated: params.clientInitiated.to_rust(),
        })
    }

    /// Sets `value` to [`field@SubscrCreateParams::subscr_namespace`] field.
    pub fn subscr_namespace(&mut self, value: Namespace) -> &mut Self {
        self.subscr_namespace = value;
        self
    }

    /// Sets `value` to [`field@SubscrCreateParams::protocol`] field.
    pub fn protocol(&mut self, value: Protocol) -> &mut Self {
        self.protocol = value;
        self
    }

    /// Sets `value` to [`field@SubscrCreateParams::qos`] field.
    pub fn qos(&mut self, value: Qos) -> &mut Self {
        self.qos = value;
        self
    }

    /// Sets `value` to [`field@SubscrCreateParams::operations`] field.
    pub fn operations(&mut self, value: OpCode) -> &mut Self {
        self.operations = value;
        self
    }

    /// Sets `value` to [`field@SubscrCreateParams::port_number`] field.
    pub fn port_number<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<u32>,
    {
        self.port_number = value.into();
        self
    }

    /// Sets `value` to [`field@SubscrCreateParams::timeout`] field.
    pub fn timeout(&mut self, value: Duration) -> &mut Self {
        self.timeout = value;
        self
    }

    /// Sets `value` to [`field@SubscrCreateParams::name`] field.
    pub fn name<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.name = value.into();
        self
    }

    /// Sets `value` to [`field@SubscrCreateParams::callback`] field.
    pub fn callback<T>(&mut self, value: T) -> &mut Self
    where
        T: Fn(Result<Message>) + Send + Sync + 'static,
    {
        self.callback = Some(Arc::new(value));
        self
    }

    /// Sets `value` to [`field@SubscrCreateParams::recipient_name`] field.
    pub fn recipient_name<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.recipient_name = value.into();
        self
    }

    /// Sets `value` to [`field@SubscrCreateParams::ip_address`] field.
    pub fn ip_address<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.ip_address = value.into();
        self
    }

    /// Sets `value` to [`field@SubscrCreateParams::grouping_class`] field.
    pub fn grouping_class(&mut self, value: GroupingClass) -> &mut Self {
        self.grouping_class = value;
        self
    }

    /// Sets `value` to [`field@SubscrCreateParams::grouping_value`] field.
    pub fn grouping_value(&mut self, value: u32) -> &mut Self {
        self.grouping_value = value;
        self
    }

    /// Sets `value` to [`field@SubscrCreateParams::grouping_type`] field.
    pub fn grouping_type(&mut self, value: GroupingType) -> &mut Self {
        self.grouping_type = value;
        self
    }

    /// Sets `value` to [`field@SubscrCreateParams::client_initiated`] field.
    pub fn client_initiated(&mut self, value: bool) -> &mut Self {
        self.client_initiated = value;
        self
    }
}

pub(crate) struct DpiSubscrCreateParams<'a> {
    dpi_params: dpiSubscrCreateParams,
    params: &'a mut SubscrCreateParams,
    pub(crate) callback_context: Option<Arc<SubscrCallbackContext>>,
}

impl DpiSubscrCreateParams<'_> {
    pub(crate) fn new(params: &mut SubscrCreateParams) -> Result<DpiSubscrCreateParams> {
        let callback_context = params.callback.as_ref().map(SubscrCallbackContext::new);
        Ok(DpiSubscrCreateParams {
            dpi_params: dpiSubscrCreateParams {
                subscrNamespace: params.subscr_namespace.to_dpi(),
                protocol: params.protocol.to_dpi(),
                qos: params.qos.to_dpi(),
                operations: params.operations.to_dpi(),
                portNumber: params.port_number,
                timeout: duration_to_secs(params.timeout, "subscription timeout")?,
                name: params.name.to_ptr(),
                nameLength: params.name.try_to_len()?,
                callback: if callback_context.is_some() {
                    Some(SubscrCallbackContext::c_callback)
                } else {
                    None
                },
                callbackContext: if let Some(ctxt) = &callback_context {
                    ctxt.c_callback_context()
                } else {
                    ptr::null_mut()
                },
                recipientName: params.recipient_name.to_ptr(),
                recipientNameLength: params.recipient_name.try_to_len()?,
                ipAddress: params.ip_address.to_ptr(),
                ipAddressLength: params.ip_address.try_to_len()?,
                groupingClass: params.grouping_class.to_dpi(),
                groupingValue: params.grouping_value,
                groupingType: params.grouping_type.to_dpi(),
                outRegId: params.out_reg_id,
                clientInitiated: params.client_initiated.to_dpi(),
            },
            params,
            callback_context,
        })
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut dpiSubscrCreateParams {
        &mut self.dpi_params
    }

    pub(crate) fn callback_context(&mut self) -> Option<Arc<SubscrCallbackContext>> {
        self.callback_context.take()
    }

    pub(crate) fn update_out_params(&mut self) {
        self.params.out_reg_id = self.dpi_params.outRegId;
    }
}
