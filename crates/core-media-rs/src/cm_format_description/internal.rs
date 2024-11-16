use std::ffi::c_void;
use std::ptr;

use core_foundation::base::{CFAllocatorRef, CFTypeID, OSStatus, TCFType};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::{declare_TCFType, impl_CFTypeDescription, impl_TCFType};
use core_utils_rs::four_char_code::FourCharCode;

use crate::cm_format_description::error::NO_ERROR;
use crate::cm_format_description::media_type::CMMediaType;

use super::error::CMFormatDescriptionError;

#[repr(C)]
pub struct __CMFormatDescriptionRef(c_void);

pub type CMFormatDescriptionRef = *mut __CMFormatDescriptionRef;

declare_TCFType! {CMFormatDescription, CMFormatDescriptionRef}
impl_TCFType!(
    CMFormatDescription,
    CMFormatDescriptionRef,
    CMFormatDescriptionGetTypeID
);
impl_CFTypeDescription!(CMFormatDescription);

extern "C" {
    pub fn CMFormatDescriptionGetTypeID() -> CFTypeID;
}

impl CMFormatDescription {
    pub(super) fn internal_create(
        allocator: CFAllocatorRef,
        media_type: CMMediaType,
        media_sub_type: FourCharCode,
        extensions: Option<CFDictionary>,
    ) -> Result<Self, CMFormatDescriptionError> {
        extern "C" {
            fn CMFormatDescriptionCreate(
                allocator: CFAllocatorRef,
                media_type: CMMediaType,
                media_sub_type: FourCharCode,
                extensions: CFDictionaryRef,
                format_description_out: &mut CMFormatDescriptionRef,
            ) -> OSStatus;
        }
        let mut cm_format_description_ref: CMFormatDescriptionRef = ptr::null_mut();
        unsafe {
            let result = CMFormatDescriptionCreate(
                allocator,
                media_type,
                media_sub_type,
                extensions
                    .map(|e| e.clone().as_concrete_TypeRef())
                    .unwrap_or(ptr::null_mut()),
                &mut cm_format_description_ref,
            );
            if result == NO_ERROR {
                Ok(CMFormatDescription::wrap_under_create_rule(
                    cm_format_description_ref,
                ))
            } else {
                Err(CMFormatDescriptionError::from(result))
            }
        }
    }
}
