use std::ffi::c_void;

use core_foundation::base::{TCFType, TCFTypeRef};

use crate::cm_sample_buffer::CMSampleBufferRef;

use super::CMSampleBuffer;

impl CMSampleBuffer {
    pub(crate) fn internal_get_attachement<T: TCFType>(&self, key: &str) -> Option<T> {
        extern "C" {
            pub fn CMGetAttachment(
                sbuf: CMSampleBufferRef,
                key: *const u8,
                attachment_mode_out: *mut u32,
            ) -> *const c_void;
        }

        let ptr = unsafe { CMGetAttachment(self.as_concrete_TypeRef(), key.as_ptr(), std::ptr::null_mut()) };

        if ptr.is_null() {
            None
        } else {
            unsafe {
                let t_ref = T::Ref::from_void_ptr(ptr);
                Some(T::wrap_under_get_rule(t_ref))
             }
        }
    }
}
