use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

dpi_bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc]
    pub struct CreateMode: dpiCreateMode {
        #[odpic_doc]
        const DEFAULT = DPI_MODE_CREATE_DEFAULT;
        #[odpic_doc]
        const EVENTS = DPI_MODE_CREATE_EVENTS;
        #[odpic_doc]
        const THREADED = DPI_MODE_CREATE_THREADED;
    }
}
