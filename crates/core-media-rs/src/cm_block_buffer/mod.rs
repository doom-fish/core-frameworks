pub mod error;
pub mod flags;
mod internal;

use core_foundation::base::CFAllocatorRef;
pub use internal::{CMBlockBuffer, CMBlockBufferRef};

impl CMBlockBuffer {
    pub fn create_empty(
        allocator: CFAllocatorRef,
        capacity: u32,
        flags: flags::CMBlockBufferFlags,
    ) -> Result<CMBlockBuffer, error::CMBlockBufferError> {
        CMBlockBuffer::internal_create_empty(allocator, capacity, flags)
    }
}
