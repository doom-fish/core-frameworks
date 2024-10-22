use core_foundation::string::CFStringRef;

extern "C" {

    // constants
    pub static kCVImageBufferYCbCrMatrix_ITU_R_2020: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_P3_D65: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_ITU_R_709_2: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_ITU_R_601_4: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_SMPTE_240M_1995: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_DCI_P3: CFStringRef;
}
