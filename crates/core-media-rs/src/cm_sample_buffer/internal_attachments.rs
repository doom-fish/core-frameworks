use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::TCFType,
};

use crate::cm_sample_buffer::CMSampleBufferRef;

use super::{error::CMSampleBufferError, CMSampleBuffer};

impl CMSampleBuffer {
    pub fn internal_get_attachements_array<T>(&self) -> Result<CFArray<T>, CMSampleBufferError>
    where
        T: TCFType,
    {
        extern "C" {
            pub fn CMSampleBufferGetSampleAttachmentsArray(
                sample: CMSampleBufferRef,
                create: u8,
            ) -> CFArrayRef;
        }
        let attachments_ref =
            unsafe { CMSampleBufferGetSampleAttachmentsArray(self.as_concrete_TypeRef(), 1) };
        if attachments_ref.is_null() {
            Err(CMSampleBufferError::CouldNotGetDataBuffer)
        } else {
            Ok(unsafe { CFArray::wrap_under_create_rule(attachments_ref) })
        }
    }
}
