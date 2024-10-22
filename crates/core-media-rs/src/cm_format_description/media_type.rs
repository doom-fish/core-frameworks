use std::ops::Deref;

use core_utils_rs::four_char_code::FourCharCode;

#[repr(transparent)]
pub struct CMMediaType(FourCharCode);
impl CMMediaType {
    pub const VIDEO: Self = Self::from_four_char_code(*b"vide");
    pub const SOUND: Self = Self::from_four_char_code(*b"soun");
    pub const MUXED: Self = Self::from_four_char_code(*b"muxx");
    pub const TEXT: Self = Self::from_four_char_code(*b"text");
    pub const CLOSED_CAPTION: Self = Self::from_four_char_code(*b"clcp");
    pub const SUBTITLE: Self = Self::from_four_char_code(*b"sbtl");
    pub const TIME_CODE: Self = Self::from_four_char_code(*b"tmcd");
    pub const METADATA: Self = Self::from_four_char_code(*b"meta");
    pub const TAGGED_BUFFER_GROUP: Self = Self::from_four_char_code(*b"tbgr");
    // Unsafe: unsafe is ok here as we are only using predefined constants
    const fn from_four_char_code(four_char_code: [u8; 4]) -> Self {
        Self(unsafe { FourCharCode::new_unchecked(u32::from_be_bytes(four_char_code)) })
    }
}

impl Deref for CMMediaType {
    type Target = FourCharCode;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<CMMediaType> for FourCharCode {
    fn from(media_type: CMMediaType) -> FourCharCode {
        media_type.0
    }
}
