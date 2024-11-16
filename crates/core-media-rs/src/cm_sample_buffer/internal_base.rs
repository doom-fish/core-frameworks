use core_foundation::{base::TCFType, declare_TCFType, impl_CFTypeDescription, impl_TCFType};
use std::ffi::c_void;

use core_foundation::base::CFTypeID;

#[repr(C)]
pub struct __CMSampleBufferRef(c_void);

pub type CMSampleBufferRef = *mut __CMSampleBufferRef;

declare_TCFType!(CMSampleBuffer, CMSampleBufferRef);
impl_TCFType!(CMSampleBuffer, CMSampleBufferRef, CMSampleBufferGetTypeID);
impl_CFTypeDescription!(CMSampleBuffer);

unsafe impl Send for CMSampleBuffer {}

extern "C" {
    pub fn CMSampleBufferGetTypeID() -> CFTypeID;
}
