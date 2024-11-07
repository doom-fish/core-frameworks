use std::ops::Deref;

use core_foundation::{
    array::CFArray,
    base::{CFTypeID, CFTypeRef, TCFTypeRef},
};
use thiserror::Error;

use super::CMSampleBufferAttachment;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CMSampleBufferAttachments(CFArray);

impl Deref for CMSampleBufferAttachments {
    type Target = CFArray;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CMSampleBufferAttachmentError {
    #[error("Invalid type {0}")]
    InvalidType(String),
    #[error("Index: {0} is out of bounds: {1}")]
    InvalidIndex(usize, isize),
    #[error("Could not get attachment with index: {0}")]
    CouldNotGetAttachment(usize),
}

impl CMSampleBufferAttachments {
    pub fn new(attachments: CFArray) -> Self {
        CMSampleBufferAttachments(attachments)
    }
    pub fn get<T>(&self, index: usize) -> Result<T, CMSampleBufferAttachmentError>
    where
        T: CMSampleBufferAttachment,
    {
        extern "C" {
            fn CFGetTypeID(obj: CFTypeRef) -> CFTypeID;
        }
        if index as isize >= self.0.len() {
            return Err(CMSampleBufferAttachmentError::InvalidIndex(
                index,
                self.0.len(),
            ));
        }

        let Some(raw) = self.0.get(index as isize) else {
            return Err(CMSampleBufferAttachmentError::CouldNotGetAttachment(index));
        };
        let void_ptr = raw.as_void_ptr();
        let type_id = unsafe { CFGetTypeID(void_ptr) };
        if T::correct_type_invariant(type_id) {
            let t_ref = unsafe { T::Ref::from_void_ptr(void_ptr) };
            Ok(unsafe { T::wrap_under_get_rule(t_ref) })
        } else {
            Err(CMSampleBufferAttachmentError::InvalidType(
                std::any::type_name::<T>().to_string(),
            ))
        }
    }
}
