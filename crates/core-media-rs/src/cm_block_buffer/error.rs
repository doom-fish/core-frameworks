use core_foundation::base::OSStatus;

extern "C" {
    pub static kCMBlockBufferNoErr: OSStatus;
}

use thiserror::Error;

pub const NO_ERROR: OSStatus = 0;
const FLAG_OFFSET: OSStatus = -12700;
const BAD_CUSTOM_BLOCK_SOURCE: OSStatus = FLAG_OFFSET - 1;
const BAD_LENGTH_PARAMETER: OSStatus = FLAG_OFFSET - 2;
const BAD_POINTER_PARAMETER: OSStatus = FLAG_OFFSET - 3;
const BLOCK_ALLOCATION_FAILED: OSStatus = FLAG_OFFSET - 4;
const EMPTY_BBUF: OSStatus = FLAG_OFFSET - 5;
const INSUFFICIENT_SPACE: OSStatus = FLAG_OFFSET - 6;
const STRUCTURE_ALLOCATION_FAILED: OSStatus = FLAG_OFFSET - 7;
const UNALLOCATED_BLOCK: OSStatus = FLAG_OFFSET - 8;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CMBlockBufferError<TUnknown = OSStatus> {
    #[error("An error code that indicates the custom block source is invalid.")]
    BadCustomBlockSource,
    #[error("An error code that indicates the offset doesn’t point to the location of data in the memory block.")]
    BadLengthParameter,
    #[error("An error code that indicates the block buffer reference is invalid.")]
    BadPointerParameter,
    #[error("An error code that indicates the block allocator failed to allocate a memory block.")]
    BlockAllocationFailed,
    #[error("An error code that indicates the block buffer is empty.")]
    EmptyBBuf,
    #[error("An error code that indicates the system failed to create a new buffer because of insufficient space at the buffer out location.")]
    InsufficientSpace,
    #[error(
        "An error code that indicates the structure allocator failed to allocate a block buffer."
    )]
    StructureAllocationFailed,
    #[error("An error code that indicates the system encountered an unallocated memory block.")]
    UnallocatedBlock,
    #[error("An unknown error occurred with code {0}")]
    UnknownError(TUnknown),
}

impl From<OSStatus> for CMBlockBufferError {
    fn from(value: OSStatus) -> Self {
        match value {
            BAD_CUSTOM_BLOCK_SOURCE => CMBlockBufferError::BadCustomBlockSource,
            BAD_LENGTH_PARAMETER => CMBlockBufferError::BadLengthParameter,
            BAD_POINTER_PARAMETER => CMBlockBufferError::BadPointerParameter,
            BLOCK_ALLOCATION_FAILED => CMBlockBufferError::BlockAllocationFailed,
            EMPTY_BBUF => CMBlockBufferError::EmptyBBuf,
            INSUFFICIENT_SPACE => CMBlockBufferError::InsufficientSpace,
            STRUCTURE_ALLOCATION_FAILED => CMBlockBufferError::StructureAllocationFailed,
            UNALLOCATED_BLOCK => CMBlockBufferError::UnallocatedBlock,
            NO_ERROR => CMBlockBufferError::UnknownError(NO_ERROR),
            _ => CMBlockBufferError::UnknownError(value),
        }
    }
}
impl From<CMBlockBufferError> for OSStatus {
    fn from(value: CMBlockBufferError) -> Self {
        match value {
            CMBlockBufferError::BadCustomBlockSource => BAD_CUSTOM_BLOCK_SOURCE,
            CMBlockBufferError::BadLengthParameter => BAD_LENGTH_PARAMETER,
            CMBlockBufferError::BadPointerParameter => BAD_POINTER_PARAMETER,
            CMBlockBufferError::BlockAllocationFailed => BLOCK_ALLOCATION_FAILED,
            CMBlockBufferError::EmptyBBuf => EMPTY_BBUF,
            CMBlockBufferError::InsufficientSpace => INSUFFICIENT_SPACE,
            CMBlockBufferError::StructureAllocationFailed => STRUCTURE_ALLOCATION_FAILED,
            CMBlockBufferError::UnallocatedBlock => UNALLOCATED_BLOCK,
            CMBlockBufferError::UnknownError(v) => v,
        }
    }
}
