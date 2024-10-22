#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct AudioChannelFlags(u32);

impl From<AudioChannelFlags> for u32 {
    fn from(value: AudioChannelFlags) -> u32 {
        value.0
    }
}
impl From<u32> for AudioChannelFlags {
    fn from(value: u32) -> AudioChannelFlags {
        AudioChannelFlags(value)
    }
}

impl AudioChannelFlags {
    pub const RECTANGULAR_COORDINATES: Self = Self(0x1);
    pub const SPHERICAL_COORDINATES: Self = Self(0x2);
    pub const METERS: Self = Self(0x3);

    pub fn new(flags: impl Into<u32>) -> Self {
        Self(flags.into())
    }
}
