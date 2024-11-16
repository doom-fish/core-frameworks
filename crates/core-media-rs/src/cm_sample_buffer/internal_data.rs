use core_foundation::base::TCFType;

use crate::{
    cm_block_buffer::{CMBlockBuffer, CMBlockBufferRef},
    cm_sample_buffer::CMSampleBufferRef,
};

use super::{error::CMSampleBufferError, CMSampleBuffer};

impl CMSampleBuffer {
    pub fn internal_get_data_buffer(&self) -> Result<CMBlockBuffer, CMSampleBufferError> {
        extern "C" {
            fn CMSampleBufferGetDataBuffer(sampleBuffer: CMSampleBufferRef) -> CMBlockBufferRef;
        }
        let block_buffer_ref = unsafe { CMSampleBufferGetDataBuffer(self.as_concrete_TypeRef()) };
        if block_buffer_ref.is_null() {
            Err(CMSampleBufferError::CouldNotGetDataBuffer)
        } else {
            Ok(unsafe { CMBlockBuffer::wrap_under_get_rule(block_buffer_ref) })
        }
    }
}
