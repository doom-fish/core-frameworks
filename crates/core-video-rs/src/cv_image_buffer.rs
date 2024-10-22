mod internal {
    use core_foundation::{base::*, declare_TCFType, impl_TCFType};
    use std::ffi::c_void;
    // type CGColorSpaceRef = CGColorSpace;
    #[repr(C)]
    pub struct __CVImageBufferRef(c_void);

    pub type CVImageBufferRef = *mut __CVImageBufferRef;

    declare_TCFType! {CVImageBuffer, CVImageBufferRef}
    impl_TCFType!(CVImageBuffer, CVImageBufferRef, CVImageBufferGetTypeID);

    extern "C" {

        fn CVImageBufferGetTypeID() -> CFTypeID;

    }
}
pub use internal::{CVImageBuffer, CVImageBufferRef};
