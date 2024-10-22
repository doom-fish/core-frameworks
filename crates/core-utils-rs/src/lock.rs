use std::{
    error::Error,
    ops::{Deref, DerefMut},
};

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
