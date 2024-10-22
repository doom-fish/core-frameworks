use crate::{
    audio_channel_layout_tag::AudioChannelLayoutTag,
    audio_stream_basic_description::AudioStreamBasicDescription,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioBufferListItem {
    audio_stream_basic_description: AudioStreamBasicDescription,
    channel_layout_tag: AudioChannelLayoutTag,
}

impl AudioBufferListItem {
    pub fn new(
        audio_stream_basic_description: AudioStreamBasicDescription,
        channel_layout_tag: AudioChannelLayoutTag,
    ) -> Self {
        Self {
            audio_stream_basic_description,
            channel_layout_tag,
        }
    }
}
