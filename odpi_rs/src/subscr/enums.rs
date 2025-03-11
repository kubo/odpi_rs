use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    #[repr(u32)]
    pub enum EventType: dpiEventType {
        Aq = DPI_EVENT_AQ,
        Dereg = DPI_EVENT_DEREG,
        None = DPI_EVENT_NONE,
        ObjChange = DPI_EVENT_OBJCHANGE,
        QueryChange = DPI_EVENT_QUERYCHANGE,
        Shutdown = DPI_EVENT_SHUTDOWN,
        ShutdownAny = DPI_EVENT_SHUTDOWN_ANY,
        Startup = DPI_EVENT_STARTUP,
    }
}

dpi_bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    pub struct OpCode: dpiOpCode {
        #[odpic_doc]
        const ALL_OPS =  DPI_OPCODE_ALL_OPS;
        #[odpic_doc]
        const ALL_ROWS =  DPI_OPCODE_ALL_ROWS;
        #[odpic_doc]
        const ALTER =  DPI_OPCODE_ALTER;
        #[odpic_doc]
        const DELETE =  DPI_OPCODE_DELETE;
        #[odpic_doc]
        const DROP =  DPI_OPCODE_DROP;
        #[odpic_doc]
        const INSERT =  DPI_OPCODE_INSERT;
        #[odpic_doc]
        const UPDATE =  DPI_OPCODE_UPDATE;
        #[odpic_doc]
        const UNKNOWN =  DPI_OPCODE_UNKNOWN;
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc(name = "dpiSubscrGroupingClass")]
    #[repr(u8)]
    pub enum GroupingClass: dpiSubscrGroupingClass {
        ClassTime = DPI_SUBSCR_GROUPING_CLASS_TIME,
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc(name = "dpiSubscrGroupingType")]
    #[repr(u8)]
    pub enum GroupingType: dpiSubscrGroupingType {
        Last = DPI_SUBSCR_GROUPING_TYPE_LAST,
        Summary = DPI_SUBSCR_GROUPING_TYPE_SUMMARY,
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc(name = "dpiSubscrNamespace")]
    #[repr(u32)]
    pub enum Namespace: dpiSubscrNamespace {
        Aq = DPI_SUBSCR_NAMESPACE_AQ,
        DbChange = DPI_SUBSCR_NAMESPACE_DBCHANGE,
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc(name = "dpiSubscrProtocol")]
    #[repr(u32)]
    pub enum Protocol: dpiSubscrProtocol {
        Callback = DPI_SUBSCR_PROTO_CALLBACK,
        Http = DPI_SUBSCR_PROTO_HTTP,
        Mail = DPI_SUBSCR_PROTO_MAIL,
        Plsql = DPI_SUBSCR_PROTO_PLSQL,
    }
}

dpi_bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc(name = "dpiSubscrQOS")]
    pub struct Qos: dpiSubscrQOS {
        #[odpic_doc]
        const BEST_EFFORT = DPI_SUBSCR_QOS_BEST_EFFORT;
        #[odpic_doc]
        const DEREG_NFY = DPI_SUBSCR_QOS_DEREG_NFY;
        #[odpic_doc]
        const QUERY = DPI_SUBSCR_QOS_QUERY;
        #[odpic_doc]
        const RELIABLE = DPI_SUBSCR_QOS_RELIABLE;
        #[odpic_doc]
        const ROWIDS = DPI_SUBSCR_QOS_ROWIDS;
    }
}
