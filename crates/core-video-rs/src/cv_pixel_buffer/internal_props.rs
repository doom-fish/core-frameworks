use core_foundation::base::TCFType;

use crate::cv_pixel_buffer::internal_base::CVPixelBufferRef;

use super::internal_base::CVPixelBuffer;


impl  CVPixelBuffer {
    pub(super) fn internal_is_planar(&self) -> bool {
        extern "C" {
            fn CVPixelBufferIsPlanar(pixel_buffer_ref: CVPixelBufferRef) -> i32;
        }

        unsafe { CVPixelBufferIsPlanar(self.as_concrete_TypeRef()) == 1 }
    }
    pub(super) fn internal_bytes_per_row(&self) -> usize {
        extern "C" {
            fn CVPixelBufferGetBytesPerRow(pixel_buffer_ref: CVPixelBufferRef) -> usize;
        }

        unsafe { CVPixelBufferGetBytesPerRow(self.as_concrete_TypeRef()) }
    }
    pub(super) fn internal_width(&self) -> usize {
        extern "C" {
            fn CVPixelBufferGetWidth(pixel_buffer_ref: CVPixelBufferRef) -> usize;
        }

        unsafe { CVPixelBufferGetWidth(self.as_concrete_TypeRef()) }
    }
    pub(super) fn internal_height(&self) -> usize {
        extern "C" {
            fn CVPixelBufferGetHeight(pixel_buffer_ref: CVPixelBufferRef) -> usize;
        }

        unsafe { CVPixelBufferGetHeight(self.as_concrete_TypeRef()) }
    }
}
