use std::{
    alloc,
    ops::{Deref, DerefMut},
    ptr::{self},
};

use core_audio_types_rs::audio_buffer_list::AudioBufferList;
use core_foundation::base::{CFAllocatorRef, OSStatus, TCFType};

use crate::{
    cm_block_buffer::{CMBlockBuffer, CMBlockBufferRef},
    cm_sample_buffer::{error::NO_ERROR, CMSampleBufferRef},
};

use super::{error::CMSampleBufferError, CMSampleBuffer};

#[allow(non_upper_case_globals)]
pub const kCMSampleBufferFlag_AudioBufferList_Assure16ByteAlignment: u32 = 1 << 0;

#[derive(Debug, Clone)]
pub struct RetainedAudioBufferList(AudioBufferList, #[allow(dead_code)] CMBlockBuffer);

impl Deref for RetainedAudioBufferList {
    type Target = AudioBufferList;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RetainedAudioBufferList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CMSampleBuffer {
    pub fn internal_get_audio_buffer_list(
        &self,
    ) -> Result<RetainedAudioBufferList, CMSampleBufferError> {
        extern "C" {
            fn CMSampleBufferGetAudioBufferListWithRetainedBlockBuffer(
                sbuf: CMSampleBufferRef,
                buffer_list_size_needed_out: *mut usize,
                buffer_list_out: *mut AudioBufferList,
                buffer_list_size: usize,
                bbuf_struct_allocator: CFAllocatorRef,
                bbuf_memory_allocator: CFAllocatorRef,
                flags: u32,
                block_buffer_out: *mut CMBlockBufferRef,
            ) -> OSStatus;
        }
        unsafe {
            let mut buffer_size = 0;
            CMSampleBufferGetAudioBufferListWithRetainedBlockBuffer(
                self.as_concrete_TypeRef(),
                &mut buffer_size,
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                &mut ptr::null_mut(),
            );
            let block_buffer = self.internal_get_data_buffer()?;
            let layout = alloc::Layout::from_size_align(buffer_size, 16)
                .map_err(|_e| CMSampleBufferError::CouldNotGetDataBuffer)?;
            let audio_buffer_list_ptr = alloc::alloc(layout).cast::<AudioBufferList>();

            let result = CMSampleBufferGetAudioBufferListWithRetainedBlockBuffer(
                self.clone().as_concrete_TypeRef(),
                ptr::null_mut(),
                audio_buffer_list_ptr as _,
                buffer_size,
                ptr::null_mut(),
                ptr::null_mut(),
                kCMSampleBufferFlag_AudioBufferList_Assure16ByteAlignment,
                &mut block_buffer.as_concrete_TypeRef(),
            );

            if result != NO_ERROR {
                Err(CMSampleBufferError::from(result))
            } else {
                let buffer_list: AudioBufferList = ptr::read(audio_buffer_list_ptr);
                Ok(RetainedAudioBufferList(buffer_list, block_buffer))
            }
        }
    }
}
