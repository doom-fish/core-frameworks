
use std::ffi::c_void;

use core_foundation::{
    base::{CFTypeID, TCFType}, declare_TCFType, impl_CFTypeDescription, impl_TCFType
};

#[repr(C)]
pub struct __CVPixelBufferRef(c_void);

pub type CVPixelBufferRef = *mut __CVPixelBufferRef;

declare_TCFType! {CVPixelBuffer, CVPixelBufferRef}
impl_TCFType!(CVPixelBuffer, CVPixelBufferRef, CVPixelBufferGetTypeID);
impl_CFTypeDescription!(CVPixelBuffer);

extern "C" {
    fn CVPixelBufferGetTypeID() -> CFTypeID;
}
