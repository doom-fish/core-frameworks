pub mod attributes;
pub mod error;
mod internal_base;
mod internal_create;
mod internal_lock;
mod internal_props;

pub mod lock;
pub mod planar_data;

use attributes::PixelBufferAttributes;
use core_utils_rs::four_char_code::FourCharCode;
use error::CVPixelBufferError;
pub use internal_base::{CVPixelBuffer, CVPixelBufferRef};
use internal_create::CVPixelBufferWithLifetime;
use io_surface::IOSurface;
use planar_data::PlanarDataPointer;

impl CVPixelBuffer {
    pub fn is_planar(&self) -> bool {
        self.internal_is_planar()
    }
    pub fn get_bytes_per_row(&self) -> u32 {
        self.internal_bytes_per_row()
    }
    pub fn get_width(&self) -> u32 {
        self.internal_width()
    }
    pub fn get_height(&self) -> u32 {
        self.internal_height()
    }

    pub fn create(
        width: u32,
        height: u32,
        pixel_format_type: FourCharCode,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<Self, CVPixelBufferError> {
        Self::internal_create(width, height, pixel_format_type, pixel_buffer_attributes)
    }

    pub fn create_with_io_surface(
        surface: &IOSurface,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<Self, CVPixelBufferError> {
        Self::internal_create_with_io_surface(surface, pixel_buffer_attributes)
    }
    pub fn create_with_planar_bytes<'a>(
        width: u32,
        height: u32,
        pixel_format_type: FourCharCode,
        data_pointer: PlanarDataPointer,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<CVPixelBufferWithLifetime<'a>, CVPixelBufferError> {
        Self::internal_create_with_planar_bytes(
            width,
            height,
            pixel_format_type,
            data_pointer,
            |_| {},
            pixel_buffer_attributes,
        )
    }

    pub fn create_with_bytes<'a>(
        width: u32,
        height: u32,
        pixel_format_type: FourCharCode,
        base_address: Vec<u8>,
        bytes_per_row: u32,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<CVPixelBufferWithLifetime<'a>, CVPixelBufferError> {
        Self::internal_create_with_bytes(
            width,
            height,
            pixel_format_type,
            base_address,
            bytes_per_row,
            |_| {},
            pixel_buffer_attributes,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn create_with_bytes_release_cb<'a, TReleaseCallback>(
        width: u32,
        height: u32,
        pixel_format_type: FourCharCode,
        base_address: Vec<u8>,
        bytes_per_row: u32,
        release_callback: TReleaseCallback,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<CVPixelBufferWithLifetime<'a>, CVPixelBufferError>
    where
        TReleaseCallback: 'a + Send + FnOnce(Vec<u8>),
    {
        Self::internal_create_with_bytes(
            width,
            height,
            pixel_format_type,
            base_address,
            bytes_per_row,
            release_callback,
            pixel_buffer_attributes,
        )
    }

    pub fn create_with_planar_bytes_release_cb<'a, TReleaseCallback>(
        width: u32,
        height: u32,
        pixel_format_type: FourCharCode,
        data_pointer: PlanarDataPointer,
        release_callback: TReleaseCallback,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<CVPixelBufferWithLifetime<'a>, CVPixelBufferError>
    where
        TReleaseCallback: 'a + Send + FnOnce(PlanarDataPointer),
    {
        Self::internal_create_with_planar_bytes(
            width,
            height,
            pixel_format_type,
            data_pointer,
            release_callback,
            pixel_buffer_attributes,
        )
    }
}
