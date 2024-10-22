#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct AudioFormatFlags(u32);

impl From<AudioFormatFlags> for u32 {
    fn from(value: AudioFormatFlags) -> u32 {
        value.0
    }
}
impl From<u32> for AudioFormatFlags {
    fn from(value: u32) -> AudioFormatFlags {
        AudioFormatFlags(value)
    }
}

impl AudioFormatFlags {
    pub const NATIVE_FLOAT_PREFERRED: Self = Self(1 << 0);
    pub const NATIVE_FLOAT_PACKED: Self = Self(1 << 1);
    pub const NATIVE_FLOAT_NON_INTERLEAVED: Self = Self(1 << 2);
    pub const NATIVE_FLOAT_NON_MIXABLE: Self = Self(1 << 3);
    pub const NATIVE_FLOAT_NON_MIXABLE_ALL: Self = Self(1 << 4);
    pub const NATIVE_FLOAT_NON_MIXABLE_BIT: Self = Self(1 << 5);
    pub const NATIVE_FLOAT_NON_MIXABLE_BIT_ALL: Self = Self(1 << 6);
    pub fn new(flags: impl Into<u32>) -> Self {
        Self(flags.into())
    }
}
