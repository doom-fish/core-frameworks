use std::ffi::c_void;
use std::ptr;

use core_foundation::base::{CFAllocatorRef, CFTypeID, OSStatus, TCFType};
use core_foundation::{declare_TCFType, impl_CFTypeDescription, impl_TCFType};

use crate::cm_block_buffer::error::{CMBlockBufferError, NO_ERROR};

use super::flags::CMBlockBufferFlags;

#[repr(C)]
pub struct __CMBlockBufferRef(c_void);

pub type CMBlockBufferRef = *mut __CMBlockBufferRef;

declare_TCFType! {CMBlockBuffer, CMBlockBufferRef}
impl_TCFType!(CMBlockBuffer, CMBlockBufferRef, CMBlockBufferGetTypeID);
impl_CFTypeDescription!(CMBlockBuffer);

extern "C" {
    pub fn CMBlockBufferGetTypeID() -> CFTypeID;
}

impl CMBlockBuffer {
    pub(super) fn internal_create_empty(
        allocator: CFAllocatorRef,
        capacity: u32,
        flags: CMBlockBufferFlags,
    ) -> Result<CMBlockBuffer, CMBlockBufferError> {
        extern "C" {
            fn CMBlockBufferCreateEmpty(
                allocator: CFAllocatorRef,
                capacity: u32,
                flags: CMBlockBufferFlags,
                blockBufferOut: &mut CMBlockBufferRef,
            ) -> OSStatus;
        }
        let mut block_buffer_ref: CMBlockBufferRef = ptr::null_mut();
        unsafe {
            let result = CMBlockBufferCreateEmpty(
                allocator,
                capacity,
                flags,
                &mut block_buffer_ref,
            );
            if result == NO_ERROR {
                Ok(CMBlockBuffer::wrap_under_create_rule(block_buffer_ref))
            } else {
                Err(CMBlockBufferError::from(result))
            }
        }
    }
}
