use core_foundation::base::TCFType;

use crate::cv_pixel_buffer::internal_base::CVPixelBufferRef;

use super::internal_base::CVPixelBuffer;

impl CVPixelBuffer {
    pub(super) fn internal_is_planar(&self) -> bool {
        extern "C" {
            fn CVPixelBufferIsPlanar(pixel_buffer_ref: CVPixelBufferRef) -> i32;
        }

        unsafe { CVPixelBufferIsPlanar(self.as_concrete_TypeRef()) == 1 }
    }
    pub(super) fn internal_bytes_per_row(&self) -> u32 {
        extern "C" {
            fn CVPixelBufferGetBytesPerRow(pixel_buffer_ref: CVPixelBufferRef) -> u32;
        }

        unsafe { CVPixelBufferGetBytesPerRow(self.as_concrete_TypeRef()) }
    }
    pub(super) fn internal_bytes_per_row_of_plane(&self, plane_index: u32) -> u32 {
        extern "C" {
            fn CVPixelBufferGetBytesPerRowOfPlane(
                pixel_buffer_ref: CVPixelBufferRef,
                plane_index: u32,
            ) -> u32;
        }

        unsafe { CVPixelBufferGetBytesPerRowOfPlane(self.as_concrete_TypeRef(), plane_index) }
    }
    pub(super) fn internal_width(&self) -> u32 {
        extern "C" {
            fn CVPixelBufferGetWidth(pixel_buffer_ref: CVPixelBufferRef) -> u32;
        }

        unsafe { CVPixelBufferGetWidth(self.as_concrete_TypeRef()) }
    }
    pub(super) fn internal_height(&self) -> u32 {
        extern "C" {
            fn CVPixelBufferGetHeight(pixel_buffer_ref: CVPixelBufferRef) -> u32;
        }

        unsafe { CVPixelBufferGetHeight(self.as_concrete_TypeRef()) }
    }
    pub(super) fn internal_get_plane_count(&self) -> u32 {
        extern "C" {
            fn CVPixelBufferGetPlaneCount(pixel_buffer_ref: CVPixelBufferRef) -> u32;
        }

        unsafe { CVPixelBufferGetPlaneCount(self.as_concrete_TypeRef()) }
    }
}
