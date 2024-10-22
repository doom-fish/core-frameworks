pub mod aspect_ratio;
pub mod display_dimensions;

use core_foundation::string::CFStringRef;

extern "C" {

    // attachement keys
    pub static kCVImageBufferCGColorSpaceKey: CFStringRef;
    pub static kCVImageBufferCleanApertureKey: CFStringRef;
    pub static kCVImageBufferPreferredCleanApertureKey: CFStringRef;
    pub static kCVImageBufferFieldCountKey: CFStringRef;
    pub static kCVImageBufferFieldDetailKey: CFStringRef;
    pub static kCVImageBufferPixelAspectRatioKey: CFStringRef;
    pub static kCVImageBufferDisplayDimensionsKey: CFStringRef;
    pub static kCVImageBufferGammaLevelKey: CFStringRef;
    pub static kCVImageBufferICCProfileKey: CFStringRef;
    pub static kCVImageBufferYCbCrMatrixKey: CFStringRef;
    pub static kCVImageBufferColorPrimariesKey: CFStringRef;
    pub static kCVImageBufferTransferFunctionKey: CFStringRef;
    pub static kCVImageBufferChromaLocationTopFieldKey: CFStringRef;
    pub static kCVImageBufferChromaLocationBottomFieldKey: CFStringRef;
    pub static kCVImageBufferChromaSubsamplingKey: CFStringRef;
    pub static kCVImageBufferAlphaChannelIsOpaque: CFStringRef;
    pub static kCVImageBufferContentLightLevelInfoKey: CFStringRef;
    pub static kCVImageBufferMasteringDisplayColorVolumeKey: CFStringRef;

}
