use core_utils_rs::four_char_code::FourCharCode;

extern "C" {
    static kCMVideoCodecType_422YpCbCr8: u32;
    static kCMVideoCodecType_Animation: u32;
    static kCMVideoCodecType_Cinepak: u32;
    static kCMVideoCodecType_JPEG: u32;
    static kCMVideoCodecType_JPEG_OpenDML: u32;
    static kCMVideoCodecType_SorensonVideo: u32;
    static kCMVideoCodecType_SorensonVideo3: u32;
    static kCMVideoCodecType_H263: u32;
    static kCMVideoCodecType_H264: u32;
    static kCMVideoCodecType_MPEG4Video: u32;
    static kCMVideoCodecType_MPEG2Video: u32;
    static kCMVideoCodecType_MPEG1Video: u32;
    static kCMVideoCodecType_DVCNTSC: u32;
    static kCMVideoCodecType_DVCPAL: u32;
    static kCMVideoCodecType_DVCProPAL: u32;
    static kCMVideoCodecType_DVCPro50NTSC: u32;
    static kCMVideoCodecType_DVCPro50PAL: u32;
    static kCMVideoCodecType_DVCPROHD720p60: u32;
    static kCMVideoCodecType_DVCPROHD720p50: u32;
    static kCMVideoCodecType_DVCPROHD1080i60: u32;
    static kCMVideoCodecType_DVCPROHD1080i50: u32;
    static kCMVideoCodecType_DVCPROHD1080p30: u32;
    static kCMVideoCodecType_DVCPROHD1080p25: u32;
}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CMVideoCodecType(FourCharCode);

impl CMVideoCodecType {
    const fn new(code: u32) -> Self {
        CMVideoCodecType(unsafe { FourCharCode::new_unchecked(code) })
    }
    // A type that identifies a component with the format of Y’CbCr 8-bit 4:2:2 ordered Cb Y’0 Cr Y’1.
    pub const YP_CB_CR_8_422: Self = Self::new(unsafe { kCMVideoCodecType_422YpCbCr8 });
    // A type that identifies the apple animation format.
    pub const ANIMATION: Self = Self::new(unsafe { kCMVideoCodecType_Animation });
    // A type that identifies the cinepak format.
    pub const CINEPAK: Self = Self::new(unsafe { kCMVideoCodecType_Cinepak });
    // A type that identifies the sorenson video format.
    pub const SORENSON_VIDEO: Self = Self::new(unsafe { kCMVideoCodecType_SorensonVideo });
    // A type that identifies the sorenson 3 video format.
    pub const SORENSON_VIDEO3: Self = Self::new(unsafe { kCMVideoCodecType_SorensonVideo3 });
    // A type that identifies the Joint Photographic Experts Group (JPEG) format.
    pub const JPEG: Self = Self::new(unsafe { kCMVideoCodecType_JPEG });
    // A type that identifies the JPEG format with Open-DML extensions.
    pub const JPEG_OPEN_DML: Self = Self::new(unsafe { kCMVideoCodecType_JPEG_OpenDML });
    // A type that identifies the ITU-T H.263 format.
    pub const H263: Self = Self::new(unsafe { kCMVideoCodecType_H263 });
    // A type that identifies the ITU-T H.264 format.
    pub const H264: Self = Self::new(unsafe { kCMVideoCodecType_H264 });
    // A type that identifies the MPEG-1 video format.
    pub const MPEG1_VIDEO: Self = Self::new(unsafe { kCMVideoCodecType_MPEG1Video });
    // A type that identifies the MPEG-2 video format.
    pub const MPEG2_VIDEO: Self = Self::new(unsafe { kCMVideoCodecType_MPEG2Video });
    // A type that identifies the Moving Picture Experts Group (MPEG) MPEG-4 Part 2 video format.
    pub const MPEG4_VIDEO: Self = Self::new(unsafe { kCMVideoCodecType_MPEG4Video });
    // A type that identifies the DV NTSC format.
    pub const DVC_NTSC: Self = Self::new(unsafe { kCMVideoCodecType_DVCNTSC });
    // A type that identifies the DV PAL format.
    pub const DVC_PAL: Self = Self::new(unsafe { kCMVideoCodecType_DVCPAL });
    // A type that identifies the Panasonic DVCPro PAL format.
    pub const DVC_PRO_PAL: Self = Self::new(unsafe { kCMVideoCodecType_DVCProPAL });
    // A type that identifies the Panasonic DVCPro-50 NTSC format.
    pub const DVC_PRO50_NTSC: Self = Self::new(unsafe { kCMVideoCodecType_DVCPro50NTSC });
    // A type that identifies the Panasonic DVCPro-50 PAL format.
    pub const DVC_PRO50_PAL: Self = Self::new(unsafe { kCMVideoCodecType_DVCPro50PAL });
    // A type that identifies the Panasonic DVCPro-HD 720p60 format.
    pub const DVC_PRO_HD720P60: Self = Self::new(unsafe { kCMVideoCodecType_DVCPROHD720p60 });
    // A type that identifies the Panasonic DVCPro-HD 720p50 format.
    pub const DVC_PRO_HD720P50: Self = Self::new(unsafe { kCMVideoCodecType_DVCPROHD720p50 });
    // A type that identifies the Panasonic DVCPro-HD 1080i60 format.
    pub const DVC_PROHD_1080I60: Self = Self::new(unsafe { kCMVideoCodecType_DVCPROHD1080i60 });
    // A type that identifies the Panasonic DVCPro-HD 1080i50 format.
    pub const DVC_PROHD_1080I50: Self = Self::new(unsafe { kCMVideoCodecType_DVCPROHD1080i50 });
    // A type that identifies the Panasonic DVCPro-HD 1080p30 format.
    pub const DVC_PROHD_1080P30: Self = Self::new(unsafe { kCMVideoCodecType_DVCPROHD1080p30 });
    // A type that identifies the Panasonic DVCPro-HD 1080p25 format.
    pub const DVC_PROHD_1080P25: Self = Self::new(unsafe { kCMVideoCodecType_DVCPROHD1080p25 });
}

impl From<u32> for CMVideoCodecType {
    fn from(code: u32) -> Self {
        Self::new(code)
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
