use core_foundation::string::CFStringRef;

extern "C" {

    // fill out all constnats
    pub static kCVImageBufferColorPrimaries_ITU_R_709_2: CFStringRef;
    pub static kCVImageBufferColorPrimaries_EBU_3213: CFStringRef;
    pub static kCVImageBufferColorPrimaries_SMPTE_C: CFStringRef;
    pub static kCVImageBufferColorPrimaries_DCI_P3: CFStringRef;
    pub static kCVImageBufferColorPrimaries_ITU_R_2020: CFStringRef;
    pub static kCVImageBufferColorPrimaries_P3_D65: CFStringRef;
    pub static kCVImageBufferColorPrimaries_P22: CFStringRef;
}
