use core_utils_rs::{four_char_code::FourCharCode, four_char_code_unchecked::from_str_unchecked};
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CMVideoCodecType(FourCharCode);
impl CMVideoCodecType {
    const fn from_static_str(code: &'static str) -> Self {
        CMVideoCodecType(from_str_unchecked(code))
    }
    /// A type that identifies a component with the format of Y’CbCr 8-bit 4:2:2 ordered Cb Y’0 Cr Y’1.
    pub const YCBCR_422: Self = Self::from_static_str("2vuy");

    /// A type that identifies the apple animation format.
    pub const ANIMATION: Self = Self::from_static_str("rle ");

    /// A type that identifies the cinepak format.
    pub const CINEPAK: Self = Self::from_static_str("cvid");

    /// A type that identifies the sorenson video format.
    pub const SORENSON_VIDEO: Self = Self::from_static_str("SVQ1");
    /// A type that identifies the sorenson 3 video format.
    pub const SORENSON_VIDEO3: Self = Self::from_static_str("SVQ3");

    /// A type that identifies the Joint Photographic Experts Group (JPEG) format.
    pub const JPEG: Self = Self::from_static_str("jpeg");
    /// A type that identifies the JPEG format with Open-DML extensions.
    pub const JPEG_OPEN_DML: Self = Self::from_static_str("dmb1");

    /// A type that identifies the ITU-T H.263 format.
    pub const H263: Self = Self::from_static_str("h263");
    /// A type that identifies the ITU-T H.264 format.
    pub const H264: Self = Self::from_static_str("avc1");
    /// A type that identifies the ITU-T H.265 format.
    pub const HEVC: Self = Self::from_static_str("hvc1");
    /// A type that identifies the ITU-T H.265 format with alpha channel.
    pub const HEVC_WITH_ALPHA: Self = Self::from_static_str("muxa");
    /// A type that identifies the MPEG-1 video format.
    pub const DOLBY_VISION_HEVC: Self = Self::from_static_str("dvh1");
    /// A type that identifies the MPEG-4 video format.
    pub const MPEG4_VIDEO: Self = Self::from_static_str("mp4v");
    /// A type that identifies the MPEG-2 video format.
    pub const MPEG2_VIDEO: Self = Self::from_static_str("mp2v");
    /// A type that identifies the MPEG-1 video format.
    pub const MPEG1_VIDEO: Self = Self::from_static_str("mp1v");
    /// A type that identifies the vp9 format.
    pub const VP9: Self = Self::from_static_str("vp09");

    /// A type that identifies the DV NTSC format.
    pub const DVC_NTSC: Self = Self::from_static_str("dvc ");
    /// A type that identifies the DV PAL format.
    pub const DVC_PAL: Self = Self::from_static_str("dvcp");
    /// A type that identifies the Panasonic DVCPro PAL format.
    pub const DVC_PRO_PAL: Self = Self::from_static_str("dvpp");
    /// A type that identifies the Panasonic DVCPro-50 NTSC format.
    pub const DVC_PRO_50_NTSC: Self = Self::from_static_str("dv5n");
    /// A type that identifies the Panasonic DVCPro-50 PAL format.
    pub const DVC_PRO_50_PAL: Self = Self::from_static_str("dv5p");
    /// A type that identifies the Panasonic DVCPro-HD 720p60 format.
    pub const DVC_PRO_HD_720P60: Self = Self::from_static_str("dvhp");
    /// A type that identifies the Panasonic DVCPro-HD 720p50 format.
    pub const DVC_PRO_HD_720P50: Self = Self::from_static_str("dvhq");
    /// A type that identifies the Panasonic DVCPro-HD 1080i60 format.
    pub const DVC_PRO_HD_1080I60: Self = Self::from_static_str("dvh6");
    /// A type that identifies the Panasonic DVCPro-HD 1080i50 format.
    pub const DVC_PRO_HD_1080I50: Self = Self::from_static_str("dvh5");
    /// A type that identifies the Panasonic DVCPro-HD 1080p30 format.
    pub const DVC_PRO_HD_1080P30: Self = Self::from_static_str("dvh3");
    /// A type that identifies the Panasonic DVCPro-HD 1080p25 format.
    pub const DVC_PRO_HD_1080P25: Self = Self::from_static_str("dvh2");

    /// A type that identifies the Apple ProRes 4444 XQ format.
    pub const APPLE_PRO_RES_4444_XQ: Self = Self::from_static_str("ap4x");
    /// A type that identifies the Apple ProRes 4444 format.
    pub const APPLE_PRO_RES_4444: Self = Self::from_static_str("ap4h");
    /// A type that identifies the Apple ProRes 422 HQ format.
    pub const APPLE_PRO_RES_422_HQ: Self = Self::from_static_str("apch");
    /// A type that identifies the Apple ProRes 422 format.
    pub const APPLE_PRO_RES_422: Self = Self::from_static_str("apcn");
    /// A type that identifies the Apple ProRes 422 LT format.
    pub const APPLE_PRO_RES_422_LT: Self = Self::from_static_str("apcs");
    /// A type that identifies the Apple ProRes 422 Proxy format.
    pub const APPLE_PRO_RES_422_PROXY: Self = Self::from_static_str("apco");
    /// A type that identifies the Apple ProRes RAW format.
    pub const APPLE_PRO_RES_RAW: Self = Self::from_static_str("aprn");
    /// A type that identifies the Apple ProRes RAW HQ format.
    pub const APPLE_PRO_RES_RAW_HQ: Self = Self::from_static_str("aprh");

    /// A type that identifies the Disparity HVEC format.
    pub const DISPARITY_HEVC: Self = Self::from_static_str("dish");
    /// A type that identifies the Depth HVEC format.
    pub const DEPTH_HEVC: Self = Self::from_static_str("deph");

    /// A type that identifies the AV1 format.
    pub const AV1: Self = Self::from_static_str("av01");
}

impl From<u32> for CMVideoCodecType {
    fn from(code: u32) -> Self {
        Self(unsafe { FourCharCode::new_unchecked(code) })
    }
}

impl From<FourCharCode> for CMVideoCodecType {
    fn from(code: FourCharCode) -> Self {
        CMVideoCodecType(code)
    }
}

impl From<CMVideoCodecType> for FourCharCode {
    fn from(code: CMVideoCodecType) -> Self {
        code.0
    }
}

impl From<CMVideoCodecType> for u32 {
    fn from(code: CMVideoCodecType) -> Self {
        code.0.into()
    }
}
