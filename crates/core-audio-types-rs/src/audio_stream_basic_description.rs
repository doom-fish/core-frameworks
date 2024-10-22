use crate::{audio_format_flags::AudioFormatFlags, audio_format_id::AudioFormatID};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioStreamBasicDescription {
    pub format_id: AudioFormatID,
    pub format_flags: AudioFormatFlags,
    pub sample_rate: f64,
    pub bits_per_channel: u32,
    pub bytes_per_frame: u32,
    pub channels_per_frame: u32,
    pub bytes_per_packet: u32,
    pub frames_per_packet: u32,
    pub reserved: u32,
}

impl AudioStreamBasicDescription {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        format_id: AudioFormatID,
        format_flags: AudioFormatFlags,
        sample_rate: f64,
        bits_per_channel: u32,
        bytes_per_frame: u32,
        channels_per_frame: u32,
        bytes_per_packet: u32,
        frames_per_packet: u32,
        reserved: u32,
    ) -> Self {
        Self {
            format_id,
            format_flags,
            sample_rate,
            bits_per_channel,
            bytes_per_frame,
            channels_per_frame,
            bytes_per_packet,
            frames_per_packet,
            reserved,
        }
    }
}
