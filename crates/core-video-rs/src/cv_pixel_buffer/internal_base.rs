use core::fmt;

use std::{ffi::c_void, fmt::Formatter};

use core_foundation::{base::{CFTypeID, TCFType}, declare_TCFType, impl_TCFType};


#[repr(C)]
pub struct __CVPixelBufferRef(c_void);

pub type CVPixelBufferRef = *mut __CVPixelBufferRef;

declare_TCFType! {CVPixelBuffer, CVPixelBufferRef}
impl_TCFType!(CVPixelBuffer, CVPixelBufferRef, CVPixelBufferGetTypeID);

impl fmt::Debug for CVPixelBuffer {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "CVPixelBuffer")
    }
}

extern "C" {
    fn CVPixelBufferGetTypeID() -> CFTypeID;
}

