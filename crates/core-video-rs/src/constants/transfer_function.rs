use core_foundation::string::CFStringRef;

extern "C" {

    // fill out all constnats
    pub static kCVImageBufferTransferFunction_ITU_R_709_2: CFStringRef;
    pub static kCVImageBufferTransferFunction_SMPTE_240M_1995: CFStringRef;
    pub static kCVImageBufferTransferFunction_UseGamma: CFStringRef;
    pub static kCVImageBufferTransferFunction_ITU_R_2020: CFStringRef;
    pub static kCVImageBufferTransferFunction_SMPTE_ST_428_1: CFStringRef;
    pub static kCVImageBufferTransferFunction_ITU_R_2100_HLG: CFStringRef;
    pub static kCVImageBufferTransferFunction_SMPTE_ST_2084_PQ: CFStringRef;
    pub static kCVImageBufferTransferFunction_EBU_3213: CFStringRef;
    pub static kCVImageBufferTransferFunction_SMPTE_C: CFStringRef;
}
