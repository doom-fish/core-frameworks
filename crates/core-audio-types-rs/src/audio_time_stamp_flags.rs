#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct AudioTimeStampFlags(u32);

impl From<AudioTimeStampFlags> for u32 {
    fn from(value: AudioTimeStampFlags) -> u32 {
        value.0
    }
}
impl From<u32> for AudioTimeStampFlags {
    fn from(value: u32) -> AudioTimeStampFlags {
        AudioTimeStampFlags(value)
    }
}

impl AudioTimeStampFlags {
    pub const HOST_TIME_VALID: Self = Self(1 << 0);
    pub const RATE_SCALAR_VALID: Self = Self(1 << 1);
    pub const SAMPLE_HOST_TIME_VALID: Self = Self(1 << 2);
    pub const SAMPLE_TIME_VALID: Self = Self(1 << 3);
    pub const SMTPE_TIME_VALID: Self = Self(1 << 4);
    pub const WORD_CLOCK_TIME_VALID: Self = Self(1 << 5);

    pub fn new(flags: impl Into<u32>) -> Self {
        Self(flags.into())
    }
}
