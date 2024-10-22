use crate::{audio_channel_flags::AudioChannelFlags, audio_channel_layout::AudioChannelLabel};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioChannelDescription {
    pub channel_label: AudioChannelLabel,
    pub channel_flags: AudioChannelFlags,
    pub coordinates: [f32; 3],
}

impl AudioChannelDescription {
    pub fn new(
        channel_label: AudioChannelLabel,
        channel_flags: AudioChannelFlags,
        coordinates: [f32; 3],
    ) -> Self {
        Self {
            channel_label,
            channel_flags,
            coordinates,
        }
    }
}
