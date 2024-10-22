use core_foundation::base::{OSStatus, TCFType};

use crate::cm_sample_buffer::{
    error::{CMSampleBufferError, NO_ERROR},
    internal_base::CMSampleBuffer,
    CMSampleBufferRef,
};

impl CMSampleBuffer {
    pub(super) fn internal_make_data_ready(&self) -> Result<(), CMSampleBufferError> {
        extern "C" {
            pub fn CMSampleBufferMakeDataReady(sampleBuffer: CMSampleBufferRef) -> OSStatus;
        }
        let result = unsafe { CMSampleBufferMakeDataReady(self.as_concrete_TypeRef()) };
        if result != NO_ERROR {
            return Err(CMSampleBufferError::from(result));
        }
        Ok(())
    }
}
