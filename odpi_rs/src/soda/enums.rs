use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

dpi_bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[odpic_doc(name = "dpiSodaFlags")]
    pub struct Flags : u32 {
        #[odpic_doc]
        const ATOMIC_COMMIT = DPI_SODA_FLAGS_ATOMIC_COMMIT;
        #[odpic_doc]
        const CREATE_COLL_MAP = DPI_SODA_FLAGS_CREATE_COLL_MAP;
        #[odpic_doc]
        const DEFAULT = DPI_SODA_FLAGS_DEFAULT;
        #[odpic_doc]
        const INDEX_DROP_FORCE = DPI_SODA_FLAGS_INDEX_DROP_FORCE;
    }
}
