use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

dpi_bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    pub struct AuthMode: dpiAuthMode {
        #[odpic_doc]
        const DEFAULT = DPI_MODE_AUTH_DEFAULT;
        #[odpic_doc]
        const PRELIM = DPI_MODE_AUTH_PRELIM;
        #[odpic_doc]
        const SYSASM = DPI_MODE_AUTH_SYSASM;
        #[odpic_doc]
        const SYSBKP = DPI_MODE_AUTH_SYSBKP;
        #[odpic_doc]
        const SYSDBA = DPI_MODE_AUTH_SYSDBA;
        #[odpic_doc]
        const SYSDGD = DPI_MODE_AUTH_SYSDGD;
        #[odpic_doc]
        const SYSKMT = DPI_MODE_AUTH_SYSKMT;
        #[odpic_doc]
        const SYSOPER = DPI_MODE_AUTH_SYSOPER;
        #[odpic_doc]
        const SYSRAC = DPI_MODE_AUTH_SYSRAC;
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    #[repr(u32)]
    pub enum ConnCloseMode: dpiConnCloseMode {
        Default = DPI_MODE_CONN_CLOSE_DEFAULT,
        Drop = DPI_MODE_CONN_CLOSE_DROP,
        Retag = DPI_MODE_CONN_CLOSE_RETAG,
    }
}

dpi_bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    pub struct PoolCloseMode: dpiPoolCloseMode {
        #[odpic_doc]
        const DEFAULT = DPI_MODE_POOL_CLOSE_DEFAULT;
        #[odpic_doc]
        const FORCE = DPI_MODE_POOL_CLOSE_FORCE;
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    #[repr(u8)]
    pub enum PoolGetMode: dpiPoolGetMode {
        ForceGet = DPI_MODE_POOL_GET_FORCEGET,
        NoWait = DPI_MODE_POOL_GET_NOWAIT,
        TimedWait = DPI_MODE_POOL_GET_TIMEDWAIT,
        Wait = DPI_MODE_POOL_GET_WAIT,
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    #[repr(u32)]
    pub enum Purity: dpiPurity {
        Default = DPI_PURITY_DEFAULT,
        New = DPI_PURITY_NEW,
        Self_ = DPI_PURITY_SELF,
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    #[repr(u8)]
    pub enum ServerType: dpiServerType {
        Dedicated = DPI_SERVER_TYPE_DEDICATED,
        Pooled = DPI_SERVER_TYPE_POOLED,
        Shared = DPI_SERVER_TYPE_SHARED,
        Unknown = DPI_SERVER_TYPE_UNKNOWN,
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    #[repr(u32)]
    pub enum ShutdownMode: dpiShutdownMode {
        Abort = DPI_MODE_SHUTDOWN_ABORT,
        Default = DPI_MODE_SHUTDOWN_DEFAULT,
        Final = DPI_MODE_SHUTDOWN_FINAL,
        Immediate = DPI_MODE_SHUTDOWN_IMMEDIATE,
        Transactional = DPI_MODE_SHUTDOWN_TRANSACTIONAL,
        TransactionalLocal = DPI_MODE_SHUTDOWN_TRANSACTIONAL_LOCAL,
    }
}

dpi_bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    pub struct StartupMode : dpiStartupMode {
        #[odpic_doc]
        const DEFAULT = DPI_MODE_STARTUP_DEFAULT;
        #[odpic_doc]
        const FORCE = DPI_MODE_STARTUP_FORCE;
        #[odpic_doc]
        const RESTRICT = DPI_MODE_STARTUP_RESTRICT;
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    #[repr(u32)]
    pub enum TpcEndFlags: dpiTpcEndFlags {
        Normal = DPI_TPC_END_NORMAL,
        Suspend = DPI_TPC_END_SUSPEND,
    }
}

dpi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    #[repr(u32)]
    pub enum TpcBeginFlags: dpiTpcBeginFlags {
        Join = DPI_TPC_BEGIN_JOIN,
        New = DPI_TPC_BEGIN_NEW,
        Promote = DPI_TPC_BEGIN_PROMOTE,
        Resume = DPI_TPC_BEGIN_RESUME,
    }
}
