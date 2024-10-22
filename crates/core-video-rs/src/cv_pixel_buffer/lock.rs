use std::{
    io, ops::{Deref, DerefMut}
};

use core_utils_rs::lock::{
    self, LockGuard, LockGuardTrait, LockTrait, MutLockGuard, MutLockGuardTrait, MutLockTrait,
};

use crate::cv_pixel_buffer::{error::CVPixelBufferError, CVPixelBuffer};

use super::internal_lock::CVPixelBufferLockFlags;



#[derive(Debug)]
pub struct BaseAddressGuard<'a>(CVPixelBuffer, &'a [u8]);

impl BaseAddressGuard<'_> {
    pub fn as_slice(&self) -> &[u8] {
        self.1
    }
    pub fn as_cursor(&self) -> io::Cursor<&[u8]> {
        io::Cursor::new(self.1)
    }
}
impl Deref for BaseAddressGuard<'_> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.1
    }
}

#[derive(Debug)]
pub struct MutBaseAddressGuard<'a>(CVPixelBuffer, &'a mut [u8]);

impl MutBaseAddressGuard<'_> {
    pub fn as_slice(&self) -> &[u8] {
        self.1
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.1
    }
    pub fn as_cursor(&self) -> io::Cursor<&[u8]> {
        io::Cursor::new(self.1)
    }
    pub fn as_mut_cursor(&mut self) -> io::Cursor<&mut [u8]> {
        io::Cursor::new(self.1)
    }
}

impl Deref for MutBaseAddressGuard<'_> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.1
    }
}

impl DerefMut for MutBaseAddressGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.1
    }
}

impl LockGuardTrait for BaseAddressGuard<'_> {
    fn unlock(&self) {
        self.0
            .internal_unlock_base_address(CVPixelBufferLockFlags::ReadOnly)
            .expect("Could not unlock base");
    }
}
impl MutLockGuardTrait for MutBaseAddressGuard<'_> {
    fn unlock_mut(&mut self) {
        self.0
            .internal_unlock_base_address(CVPixelBufferLockFlags::ReadWrite)
            .expect("Could not unlock base");
    }
}

impl<'a> LockTrait<BaseAddressGuard<'a>, CVPixelBufferError> for CVPixelBuffer {
    fn lock(&self) -> Result<LockGuard<BaseAddressGuard<'a>>, CVPixelBufferError> {
        self.internal_lock_base_address(CVPixelBufferLockFlags::ReadOnly)?;
        Ok(LockGuard(BaseAddressGuard(
            self.clone(),
            self.internal_base_address()?,
        )))
    }
}
impl<'a> MutLockTrait<MutBaseAddressGuard<'a>, CVPixelBufferError> for CVPixelBuffer {
    fn lock_mut(
        &mut self,
    ) -> Result<lock::MutLockGuard<MutBaseAddressGuard<'a>>, CVPixelBufferError> {
        self.internal_lock_base_address(CVPixelBufferLockFlags::ReadWrite)?;
        Ok(MutLockGuard(MutBaseAddressGuard(
            self.clone(),
            self.internal_base_address_mut()?,
        )))
    }
}
