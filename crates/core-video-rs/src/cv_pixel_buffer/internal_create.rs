#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
use super::internal_base::CVPixelBuffer;
use crate::cv_pixel_buffer::error::CV_RETURN_SUCCESS;
use crate::cv_pixel_buffer::internal_base::CVPixelBufferRef;
use crate::types::{CVReturn, OSType};
use core_foundation::base::{kCFAllocatorDefault, CFAllocatorRef, CFType, TCFType};
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::string::CFString;
use core_graphics::display::CFDictionary;
use core_utils_rs::four_char_code::FourCharCode;
use core_utils_rs::trampoline::{create_left_trampoline, TrampolineLeftCallback, TrampolineRefcon};
use io_surface::{IOSurface, IOSurfaceRef};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::{self};

use super::attributes::PixelBufferAttributes;
use super::error::CVPixelBufferError;
use super::planar_data::PlanarDataPointer;

#[derive(Debug)]
pub struct CVPixelBufferWithLifetime<'a>(pub CVPixelBuffer, pub PhantomData<&'a ()>);

impl<'a> Deref for CVPixelBufferWithLifetime<'a> {
    type Target = CVPixelBuffer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> DerefMut for CVPixelBufferWithLifetime<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl CVPixelBuffer {
    pub(super) fn internal_create_with_planar_bytes<'a, TReleaseCallback>(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        data_pointer: PlanarDataPointer,
        release_callback: TReleaseCallback,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<CVPixelBufferWithLifetime<'a>, CVPixelBufferError>
    where
        TReleaseCallback: 'a + Send + FnOnce(PlanarDataPointer),
    {
        extern "C" {
            fn CVPixelBufferCreateWithPlanarBytes(
                allocator: CFAllocatorRef,
                width: usize,
                height: usize,
                pixel_format_type: OSType,
                data_ptr: *mut u8,
                data_size: usize,
                number_of_planes: usize,
                base_addresses: *const *const u8,
                plane_width: *const usize,
                plane_height: *const usize,
                plane_bytes_per_row: *const usize,
                release_callback: TrampolineLeftCallback,
                release_ref_con: TrampolineRefcon,
                pixel_buffer_attributes: CFDictionaryRef,
                pixel_buffer_out: *mut CVPixelBufferRef,
            ) -> CVReturn;

        }
        let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
        let data_ptr = data_pointer.as_ptr();
        let data_size = data_pointer.data_size();
        let number_of_planes = data_pointer.number_of_planes();
        let base_addresses = data_pointer.raw_base_addresses();
        let plane_width = data_pointer.plane_width();
        let plane_height = data_pointer.plane_height();
        let plane_bytes_per_row = data_pointer.plane_bytes_per_row();
        let (caller, closure) = create_left_trampoline(move |_| release_callback(data_pointer));
        let pixel_buffer_attributes: CFDictionary<CFString, CFType> =
            pixel_buffer_attributes.into();
        unsafe {
            let result = CVPixelBufferCreateWithPlanarBytes(
                kCFAllocatorDefault,
                width,
                height,
                pixel_format_type.as_u32(),
                data_ptr,
                data_size,
                number_of_planes,
                base_addresses,
                plane_width,
                plane_height,
                plane_bytes_per_row,
                caller,
                closure,
                pixel_buffer_attributes.as_concrete_TypeRef(),
                &mut pixel_buffer_out,
            );

            if result == CV_RETURN_SUCCESS {
                Ok(CVPixelBufferWithLifetime(
                    CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out),
                    PhantomData,
                ))
            } else {
                Err(CVPixelBufferError::from(result))
            }
        }
    }
    pub(super) fn internal_create_with_bytes<'a, TReleaseCallback>(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        base_address: Vec<u8>,
        bytes_per_row: usize,
        release_callback: TReleaseCallback,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<CVPixelBufferWithLifetime<'a>, CVPixelBufferError>
    where
        TReleaseCallback: 'a + Send + FnOnce(Vec<u8>),
    {
        extern "C" {
            fn CVPixelBufferCreateWithBytes(
                allocator: CFAllocatorRef,
                width: usize,
                height: usize,
                pixel_format_type: OSType,
                base_address: *const u8,
                bytes_per_row: usize,
                raw_release_callback: TrampolineLeftCallback,
                raw_release_ref_con: TrampolineRefcon,
                pixel_buffer_attributes: CFDictionaryRef,
                pixel_buffer_out: *mut CVPixelBufferRef,
            ) -> CVReturn;

        }
        if base_address.len() < bytes_per_row * height {
            return Err(CVPixelBufferError::InvalidSize);
        }

        let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
        let base_address_ptr = base_address.as_ptr();
        let (caller, closure) = create_left_trampoline(move |_| release_callback(base_address));
        unsafe {
            let pixel_buffer_attributes: CFDictionary<CFString, CFType> =
                pixel_buffer_attributes.into();
            let result = CVPixelBufferCreateWithBytes(
                kCFAllocatorDefault,
                width,
                height,
                pixel_format_type.as_u32(),
                base_address_ptr,
                bytes_per_row,
                caller,
                closure,
                pixel_buffer_attributes.as_concrete_TypeRef(),
                &mut pixel_buffer_out,
            );
            if result == CV_RETURN_SUCCESS {
                Ok(CVPixelBufferWithLifetime(CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out), PhantomData))
            } else {
                Err(CVPixelBufferError::from(result))
            }
        }
    }
    pub(super) fn internal_create_with_io_surface(
        surface: &IOSurface,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<Self, CVPixelBufferError> {
        extern "C" {
            fn CVPixelBufferCreateWithIOSurface(
                allocator: CFAllocatorRef,
                surface: IOSurfaceRef,
                pixelBufferAttributes: CFDictionaryRef,
                pixelBufferOut: *mut CVPixelBufferRef,
            ) -> CVReturn;

        }
        let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
        let pixel_buffer_attributes: CFDictionary<CFString, CFType> =
            pixel_buffer_attributes.into();
        unsafe {
            let result = CVPixelBufferCreateWithIOSurface(
                kCFAllocatorDefault,
                surface.clone().as_concrete_TypeRef(),
                pixel_buffer_attributes.as_concrete_TypeRef(),
                &mut pixel_buffer_out,
            );
            if result == CV_RETURN_SUCCESS {
                Ok(CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out))
            } else {
                Err(CVPixelBufferError::from(result))
            }
        }
    }

    pub(super) fn internal_create(
        width: usize,
        height: usize,
        pixel_format_type: FourCharCode,
        pixel_buffer_attributes: PixelBufferAttributes,
    ) -> Result<Self, CVPixelBufferError> {
        extern "C" {
            fn CVPixelBufferCreate(
                allocator: CFAllocatorRef,
                width: usize,
                height: usize,
                pixelFormatType: OSType,
                pixelBufferAttributes: CFDictionaryRef,
                pixelBufferOut: *mut CVPixelBufferRef,
            ) -> CVReturn;
        }
        let mut pixel_buffer_out: CVPixelBufferRef = ptr::null_mut();
        let pixel_buffer_attributes: CFDictionary<CFString, CFType> =
            pixel_buffer_attributes.into();
        unsafe {
            let result = CVPixelBufferCreate(
                kCFAllocatorDefault,
                width,
                height,
                pixel_format_type.as_u32(),
                pixel_buffer_attributes.as_concrete_TypeRef(),
                &mut pixel_buffer_out,
            );
            if result == CV_RETURN_SUCCESS {
                Ok(CVPixelBuffer::wrap_under_create_rule(pixel_buffer_out))
            } else {
                Err(CVPixelBufferError::from(result))
            }
        }
    }
}
