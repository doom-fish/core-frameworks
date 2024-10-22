use core_foundation::string::CFStringRef;

extern "C" {
    pub static kCVImageBufferChromaLocation_Left: CFStringRef;
    pub static kCVImageBufferChromaLocation_Center: CFStringRef;
    pub static kCVImageBufferChromaLocation_TopLeft: CFStringRef;
    pub static kCVImageBufferChromaLocation_Top: CFStringRef;
    pub static kCVImageBufferChromaLocation_BottomLeft: CFStringRef;
    pub static kCVImageBufferChromaLocation_Bottom: CFStringRef;
    pub static kCVImageBufferChromaLocation_DV420: CFStringRef;

}
