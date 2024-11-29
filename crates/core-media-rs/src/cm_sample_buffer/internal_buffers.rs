use core_foundation::base::{CFTypeRef, TCFType, TCFTypeRef};
use core_video_rs::{cv_image_buffer::CVImageBuffer, cv_pixel_buffer::CVPixelBuffer};

use crate::{
    cm_block_buffer::{CMBlockBuffer, CMBlockBufferRef},
    cm_sample_buffer::{error::CMSampleBufferError, CMSampleBufferRef},
};

use super::CMSampleBuffer;

extern "C" {
    fn CMSampleBufferGetImageBuffer(sampleBuffer: CMSampleBufferRef) -> CFTypeRef;
}
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
    pub fn internal_get_pixel_buffer(&self) -> Result<CVPixelBuffer, CMSampleBufferError> {
        let pixel_buffer_ref = unsafe { CMSampleBufferGetImageBuffer(self.as_concrete_TypeRef()) };
        if pixel_buffer_ref.is_null() {
            Err(CMSampleBufferError::CouldNotGetDataBuffer)
        } else {
            Ok(unsafe {
                let reference = TCFTypeRef::from_void_ptr(pixel_buffer_ref);
                CVPixelBuffer::wrap_under_get_rule(reference)
            })
        }
    }
    pub fn internal_get_image_buffer(&self) -> Result<CVImageBuffer, CMSampleBufferError> {
        let image_buffer_ref = unsafe { CMSampleBufferGetImageBuffer(self.as_concrete_TypeRef()) };
        if image_buffer_ref.is_null() {
            Err(CMSampleBufferError::CouldNotGetDataBuffer)
        } else {
            Ok(unsafe {
                let reference = TCFTypeRef::from_void_ptr(image_buffer_ref);
                CVImageBuffer::wrap_under_get_rule(reference)
            })
        }
    }
}
