use std::{
    error::Error,
    io,
    ops::{Deref, DerefMut},
};

use super::internal_lock::CVPixelBufferLockFlags;
use crate::cv_pixel_buffer::{error::CVPixelBufferError, CVPixelBuffer};

#[derive(Debug)]
pub struct LockGuard<T: LockGuardTrait>(pub T);
#[derive(Debug)]
pub struct MutLockGuard<T: MutLockGuardTrait>(pub T);

impl<T: LockGuardTrait> Drop for LockGuard<T> {
    fn drop(&mut self) {
        self.0.unlock();
    }
}
impl<T: MutLockGuardTrait> Drop for MutLockGuard<T> {
    fn drop(&mut self) {
        self.0.unlock_mut();
    }
}
impl<T: LockGuardTrait> Deref for LockGuard<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: MutLockGuardTrait> Deref for MutLockGuard<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: MutLockGuardTrait> DerefMut for MutLockGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

pub trait LockTrait<T: LockGuardTrait, E: Error> {
    fn lock(&self) -> Result<LockGuard<T>, E>;
}

pub trait LockGuardTrait {
    fn unlock(&self);
}

pub trait MutLockTrait<T: MutLockGuardTrait, E: Error> {
    fn lock_mut(&mut self) -> Result<MutLockGuard<T>, E>;
}
pub trait MutLockGuardTrait {
    fn unlock_mut(&mut self);
}
#[derive(Debug)]
pub struct BaseAddressGuard<'a>(CVPixelBuffer, Vec<&'a [u8]>);

impl BaseAddressGuard<'_> {
    pub fn as_slice(&self) -> &[u8] {
        self.1[0]
    }
    pub fn as_slice_plane(&self, plane: usize) -> &[u8] {
        assert!(plane < self.1.len());
        self.1[plane]
    }
    pub fn as_cursor(&self) -> io::Cursor<&[u8]> {
        io::Cursor::new(self.1[0])
    }
    pub fn as_cursor_plane(&self, plane: usize) -> io::Cursor<&[u8]> {
        assert!(plane < self.1.len());
        io::Cursor::new(self.1[plane])
    }
}
impl Deref for BaseAddressGuard<'_> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.1[0]
    }
}

#[derive(Debug)]
pub struct MutBaseAddressGuard<'a>(CVPixelBuffer, Vec<&'a mut [u8]>);

impl MutBaseAddressGuard<'_> {
    pub fn as_slice(&self) -> &[u8] {
        self.1[0]
    }
    pub fn as_slice_plane(&self, plane: usize) -> &[u8] {
        assert!(plane < self.1.len());
        self.1[plane]
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.1[0]
    }
    pub fn as_mut_slice_plane(&mut self, plane: usize) -> &mut [u8] {
        assert!(plane < self.1.len());
        self.1[plane]
    }
    pub fn as_cursor(&self) -> io::Cursor<&[u8]> {
        io::Cursor::new(self.1[0])
    }
    pub fn as_cursor_plane(&self, plane: usize) -> io::Cursor<&[u8]> {
        assert!(plane < self.1.len());
        io::Cursor::new(self.1[plane])
    }
    pub fn as_mut_cursor(&mut self) -> io::Cursor<&mut [u8]> {
        io::Cursor::new(self.1[0])
    }
    pub fn as_mut_cursor_plane(&mut self, plane: usize) -> io::Cursor<&mut [u8]> {
        assert!(plane < self.1.len());
        io::Cursor::new(self.1[plane])
    }
}

impl Deref for MutBaseAddressGuard<'_> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.1[0]
    }
}

impl DerefMut for MutBaseAddressGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.1[0]
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
        let plane_count = self.internal_get_plane_count();
        if plane_count == 0 {
            Ok(LockGuard(BaseAddressGuard(
                self.clone(),
                vec![self.internal_base_address()?],
            )))
        } else {
            let mut planes = Vec::with_capacity(plane_count as usize);
            for i in 0..plane_count {
                planes.push(self.internal_base_address_of_plane(i)?);
            }
            Ok(LockGuard(BaseAddressGuard(self.clone(), planes)))
        }
    }
}
impl<'a> MutLockTrait<MutBaseAddressGuard<'a>, CVPixelBufferError> for CVPixelBuffer {
    fn lock_mut(&mut self) -> Result<MutLockGuard<MutBaseAddressGuard<'a>>, CVPixelBufferError> {
        self.internal_lock_base_address(CVPixelBufferLockFlags::ReadWrite)?;
        let plane_count = self.internal_get_plane_count();
        if plane_count == 0 {
            Ok(MutLockGuard(MutBaseAddressGuard(
                self.clone(),
                vec![self.internal_base_address_mut()?],
            )))
        } else {
            let mut planes = Vec::with_capacity(plane_count as usize);
            for i in 0..plane_count {
                planes.push(self.internal_base_address_of_plane_mut(i)?);
            }
            Ok(MutLockGuard(MutBaseAddressGuard(self.clone(), planes)))
        }
    }
}
