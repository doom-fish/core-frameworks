#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct SMTPETimeFlags(u32);

impl From<SMTPETimeFlags> for u32 {
    fn from(value: SMTPETimeFlags) -> u32 {
        value.0
    }
}
impl From<u32> for SMTPETimeFlags {
    fn from(value: u32) -> SMTPETimeFlags {
        SMTPETimeFlags(value)
    }
}

impl SMTPETimeFlags {
    pub const RUNNING: Self = Self(1 << 0);
    pub const VALID: Self = Self(1 << 1);

    pub fn new(flags: impl Into<u32>) -> Self {
        Self(flags.into())
    }
}
