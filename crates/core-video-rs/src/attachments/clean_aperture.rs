use core_foundation::string::CFStringRef;

extern "C" {

    // attachement keys
    pub static kCVImageBufferCleanApertureWidthKey: CFStringRef;
    pub static kCVImageBufferCleanApertureHeightKey: CFStringRef;
    pub static kCVImageBufferCleanApertureHorizontalOffsetKey: CFStringRef;
    pub static kCVImageBufferCleanApertureVerticalOffsetKey: CFStringRef;
}
