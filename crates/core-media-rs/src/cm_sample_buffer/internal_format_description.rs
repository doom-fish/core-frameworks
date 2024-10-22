use core_foundation::base::TCFType;

use crate::{
    cm_format_description::{CMFormatDescription, CMFormatDescriptionRef},
    cm_sample_buffer::{
        error::CMSampleBufferError, internal_base::CMSampleBuffer, CMSampleBufferRef,
    },
};

impl CMSampleBuffer {
    pub(super) fn internal_get_format_description(
        &self,
    ) -> Result<CMFormatDescription, CMSampleBufferError> {
        extern "C" {
            pub fn CMSampleBufferGetFormatDescription(
                sampleBuffer: CMSampleBufferRef,
            ) -> CMFormatDescriptionRef;
        }
        let reference = unsafe { CMSampleBufferGetFormatDescription(self.as_concrete_TypeRef()) };
        if reference.is_null() {
            Err(CMSampleBufferError::CouldNotGetFormatDescription)
        } else {
            Ok(unsafe { CMFormatDescription::wrap_under_get_rule(reference) })
        }
    }
}
