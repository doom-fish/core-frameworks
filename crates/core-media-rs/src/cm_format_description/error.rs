use core_foundation::base::OSStatus;

extern "C" {
    pub static kCMBlockBufferNoErr: OSStatus;
}

use thiserror::Error;

pub const NO_ERROR: OSStatus = 0;
const FLAG_OFFSET: OSStatus = -12709;
const INVALID_PARAMETER: OSStatus = FLAG_OFFSET - 1;
const ALLOCATION_FAILED: OSStatus = FLAG_OFFSET - 2;
const VALUE_NOT_AVAILABLE: OSStatus = FLAG_OFFSET - 3;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CMFormatDescriptionError<TUnknown = OSStatus> {
    #[error("error that indicates that the function recieves an empty value for a parameter it requires.")]
    InvalidParameter,
    #[error("error that indicates when an allocation fails.")]
    AllocationFailed,
    #[error("error that indicates the format description doesn’t contain the value you request.")]
    ValueNotAvailable,
    #[error("An unknown error occurred with code {0}")]
    UnknownError(TUnknown),
}

impl From<OSStatus> for CMFormatDescriptionError {
    fn from(value: OSStatus) -> Self {
        match value {
            INVALID_PARAMETER => CMFormatDescriptionError::InvalidParameter,
            ALLOCATION_FAILED => CMFormatDescriptionError::AllocationFailed,
            VALUE_NOT_AVAILABLE => CMFormatDescriptionError::ValueNotAvailable,
            _ => CMFormatDescriptionError::UnknownError(value),
        }
    }
}
impl From<CMFormatDescriptionError> for OSStatus {
    fn from(value: CMFormatDescriptionError) -> Self {
        match value {
            CMFormatDescriptionError::InvalidParameter => INVALID_PARAMETER,
            CMFormatDescriptionError::AllocationFailed => ALLOCATION_FAILED,
            CMFormatDescriptionError::ValueNotAvailable => VALUE_NOT_AVAILABLE,
            CMFormatDescriptionError::UnknownError(v) => v,
        }
    }
}
