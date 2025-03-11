use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    #[repr(u32)]
    pub enum DeqMode: dpiDeqMode {
        Browse = DPI_MODE_DEQ_BROWSE,
        Locked = DPI_MODE_DEQ_LOCKED,
        Remove = DPI_MODE_DEQ_REMOVE,
        RemoveNoData = DPI_MODE_DEQ_REMOVE_NO_DATA,
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    #[repr(u32)]
    pub enum DeqNavigation: dpiDeqNavigation {
        FirstMsg = DPI_DEQ_NAV_FIRST_MSG,
        NextMsg = DPI_DEQ_NAV_NEXT_MSG,
        NextTransaction = DPI_DEQ_NAV_NEXT_TRANSACTION,
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    #[repr(u32)]
    pub enum MessageState: dpiMessageState {
        Expired = DPI_MSG_STATE_EXPIRED,
        Processed = DPI_MSG_STATE_PROCESSED,
        Ready = DPI_MSG_STATE_READY,
        Waiting = DPI_MSG_STATE_WAITING,
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    #[repr(u16)]
    pub enum MessageDeliveryMode: dpiMessageDeliveryMode {
        Buffered = DPI_MODE_MSG_BUFFERED,
        Persistent = DPI_MODE_MSG_PERSISTENT,
        PersistentOrBuffered = DPI_MODE_MSG_PERSISTENT_OR_BUFFERED,
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    #[repr(u32)]
    pub enum Visibility: dpiVisibility {
        Immediate = DPI_VISIBILITY_IMMEDIATE,
        OnCommit = DPI_VISIBILITY_ON_COMMIT,
    }
}
