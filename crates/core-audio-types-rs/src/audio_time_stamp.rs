use crate::audio_time_stamp_flags::AudioTimeStampFlags;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioTimeStamp {
    pub flags: AudioTimeStampFlags,
    pub host_time: u64,
    pub rate_scalar: f64,
    pub reserved: u32,
    pub smpte_time: u64,
    pub sample_time: f64,
    pub word_clock_time: u64,
}

impl AudioTimeStamp {
    pub fn new(
        flags: AudioTimeStampFlags,
        host_time: u64,
        rate_scalar: f64,
        reserved: u32,
        smpte_time: u64,
        sample_time: f64,
        word_clock_time: u64,
    ) -> Self {
        Self {
            flags,
            host_time,
            rate_scalar,
            reserved,
            smpte_time,
            sample_time,
            word_clock_time,
        }
    }
}
