use std::ptr;

use core_foundation::base::{OSStatus, TCFType};

use crate::{
    cm_sample_buffer::{error::NO_ERROR, internal_base::CMSampleBuffer},
    types::{CMItemCount, CMItemIndex},
};

use super::{error::CMSampleBufferError, internal_base::CMSampleBufferRef};

impl CMSampleBuffer {
    pub(super) fn internal_get_num_samples(&self) -> CMItemCount {
        extern "C" {
            pub fn CMSampleBufferGetNumSamples(sampleBuffer: CMSampleBufferRef) -> CMItemCount;
        }
        unsafe { CMSampleBufferGetNumSamples(self.as_concrete_TypeRef()) }
    }
    pub(super) fn internal_get_total_sample_size(&self) -> isize {
        extern "C" {
            pub fn CMSampleBufferGetTotalSampleSize(sampleBuffer: CMSampleBufferRef) -> isize;
        }
        unsafe { CMSampleBufferGetTotalSampleSize(self.as_concrete_TypeRef()) }
    }
    pub(super) fn internal_get_sample_size(&self, at: CMItemIndex) -> isize {
        extern "C" {
            pub fn CMSampleBufferGetSampleSize(
                sample_buffer: CMSampleBufferRef,
                at: CMItemIndex,
            ) -> isize;
        }

        unsafe { CMSampleBufferGetSampleSize(self.as_concrete_TypeRef(), at) }
    }
    pub(super) fn internal_get_sample_size_array(&self) -> Result<Vec<isize>, CMSampleBufferError> {
        extern "C" {
            pub fn CMSampleBufferGetSampleSizeArray(
                sample_buffer: CMSampleBufferRef,
                array_to_fill: *mut isize,
                entry_count: CMItemCount,
                entires_needed_out: *mut CMItemCount,
            ) -> OSStatus;
        }
        let mut entries_needed_out: CMItemCount = 0;

        let result = unsafe {
            CMSampleBufferGetSampleSizeArray(
                self.as_concrete_TypeRef(),
                ptr::null_mut(),
                0,
                &mut entries_needed_out,
            )
        };

        if result != NO_ERROR {
            return Err(CMSampleBufferError::from(result));
        }
        let mut result_vec = vec![0isize; entries_needed_out as usize];
        let result = unsafe {
            CMSampleBufferGetSampleSizeArray(
                self.as_concrete_TypeRef(),
                result_vec.as_mut_ptr(),
                entries_needed_out,
                &mut entries_needed_out,
            )
        };

        if result == NO_ERROR {
            Ok(result_vec)
        } else {
            Err(CMSampleBufferError::from(result))
        }
    }
}
