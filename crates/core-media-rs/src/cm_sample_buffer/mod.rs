#![allow(clippy::too_many_arguments)]

pub mod error;
pub(crate) mod internal_attachments;
pub(crate) mod internal_audio;
pub(crate) mod internal_base;
pub(crate) mod internal_create;
pub(crate) mod internal_data;
pub(crate) mod internal_format_description;
pub(crate) mod internal_readyness;
pub(crate) mod internal_sizes;

use core::fmt;
use std::fmt::Formatter;

use core_foundation::base::{CFAllocatorRef, TCFType};
use error::CMSampleBufferError;
use internal_audio::RetainedAudioBufferList;
pub use internal_base::{CMSampleBuffer, CMSampleBufferRef};
use internal_create::CMSampleBufferWithLifeTime;

use crate::{
    cm_block_buffer::CMBlockBuffer, cm_format_description::CMFormatDescription,
    cm_sample_timing_info::CMSampleTimingInfo, types::CMItemCount,
};

impl fmt::Debug for CMSampleBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("CMSampleBuffer").finish()
    }
}

impl CMSampleBuffer {
    pub fn create_ready(
        allocator: CFAllocatorRef,
        block_buffer: &CMBlockBuffer,
        format_description: &CMFormatDescription,
        sample_count: CMItemCount,
        sample_timings: &[CMSampleTimingInfo],
        sample_sizes: &[i64],
    ) -> Result<Self, CMSampleBufferError> {
        Self::internal_create_ready(
            allocator,
            block_buffer,
            format_description,
            sample_count,
            sample_timings,
            sample_sizes,
        )
    }

    pub fn get_attachment<T: TCFType>(&self, key: &str) -> Option<T> {
        self.internal_get_attachement(key)
    }

    pub fn create<'a, TMakeDataReadyCallback>(
        allocator: CFAllocatorRef,
        block_buffer: &CMBlockBuffer,
        data_ready: bool,
        make_data_ready: TMakeDataReadyCallback,
        format_description: &CMFormatDescription,
        sample_count: CMItemCount,
        sample_timings: &[CMSampleTimingInfo],
        sample_sizes: &[i64],
    ) -> Result<CMSampleBufferWithLifeTime<'a>, CMSampleBufferError>
    where
        TMakeDataReadyCallback:
            'a + Send + FnOnce(CMSampleBufferRef) -> Result<(), CMSampleBufferError>,
    {
        Self::internal_create(
            allocator,
            block_buffer,
            data_ready,
            make_data_ready,
            format_description,
            sample_count,
            sample_timings,
            sample_sizes,
        )
    }
}

impl CMSampleBuffer {
    pub fn get_num_samples(&self) -> CMItemCount {
        self.internal_get_num_samples()
    }
    pub fn get_total_sample_size(&self) -> isize {
        self.internal_get_total_sample_size()
    }
    pub fn get_sample_size(&self, at: CMItemCount) -> isize {
        self.internal_get_sample_size(at)
    }
    pub fn get_sample_size_array(&self) -> Result<Vec<isize>, CMSampleBufferError> {
        self.internal_get_sample_size_array()
    }
    pub fn make_data_ready(&self) -> Result<(), CMSampleBufferError> {
        self.internal_make_data_ready()
    }
    pub fn get_format_description(&self) -> Result<CMFormatDescription, CMSampleBufferError> {
        self.internal_get_format_description()
    }
    pub fn get_audio_buffer_list(&self) -> Result<RetainedAudioBufferList, CMSampleBufferError> {
        self.internal_get_audio_buffer_list()
    }
}
