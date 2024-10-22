use core_foundation::base::OSStatus;
use thiserror::Error;

pub const CV_RETURN_SUCCESS: OSStatus = 0;

// General errors

const ERROR: OSStatus = -6660;
const INVALID_ARGUMENT: OSStatus = -6661;
const ALLOCATION_FAILED: OSStatus = -6662;
const UNSUPPORTED: OSStatus = -6663;

// Displaylink errors
const INVALID_DISPLAY: OSStatus = -6670;
const DISPLAYLINK_ALREADY_RUNNING: OSStatus = -6671;
const DISPLAYLINK_NOT_RUNNING: OSStatus = -6672;
const DISPLAYLINK_CALLBACKS_NOT_SET: OSStatus = -6673;

// Buffer errors
const INVALID_PIXEL_FORMAT: OSStatus = -6680;
const INVALID_SIZE: OSStatus = -6681;
const INVALID_PIXEL_BUFFER_ATTRIBUTES: OSStatus = -6682;
const PIXEL_BUFFER_NOT_OPENGL_COMPATIBLE: OSStatus = -6683;
const PIXEL_BUFFER_NOT_METAL_COMPATIBLE: OSStatus = -6684;

// Buffer pool errors
const WOULD_EXCEED_ALLOCATION_THRESHOLD: OSStatus = -6689;
const POOL_ALLOCATION_FAILED: OSStatus = -6690;
const INVALID_POOL_ATTRIBUTES: OSStatus = -6691;
const RETRY: OSStatus = -6692;

#[derive(Error, Debug, Clone)]
pub enum CVPixelBufferError<TUnknown = OSStatus> {
    #[error("Invalid function parameter. For example, out of range or the wrong type.")]
    InvalidArgument,
    #[error("Memory allocation for a buffer or buffer pool failed.")]
    AllocationFailed,
    #[error("The requested operation isn’t supported. For example, a function is called with a buffer that doesn’t support the operation.")]
    Unsupported,
    #[error("The display specified when creating a display link is invalid.")]
    InvalidDisplay,
    #[error("The specified display link is already running.")]
    DisplayLinkAlreadyRunning,
    #[error("The specified display link is not running.")]
    DisplayLinkNotRunning,
    #[error("No callback registered for the specified display link. You must set either the output callback or both the render and display callbacks.")]
    DisplayLinkCallbacksNotSet,
    #[error("The buffer does not support the specified pixel format.")]
    InvalidPixelFormat,
    #[error("The buffer cannot support the requested buffer size (usually too big).")]
    InvalidSize,
    #[error("A buffer cannot be created with the specified attributes.")]
    InvalidPixelBufferAttributes,
    #[error("The pixel buffer is not compatible with OpenGL due to an unsupported buffer size, pixel format, or attribute.")]
    PixelBufferNotOpenGLCompatible,
    #[error("The pixel buffer is not compatible with Metal due to an unsupported buffer size, pixel format, or attribute.")]
    PixelBufferNotMetalCompatible,
    #[error("Allocation for a pixel buffer failed because the threshold value set for the kCVPixelBufferPoolAllocationThresholdKey key in the CVPixelBufferPoolCreatePixelBufferWithAuxAttributes function would be surpassed.")]
    WouldExceedAllocationThreshold,
    #[error("Allocation for a buffer pool failed, most likely due to a lack of resources. Check to make sure your parameters are in range.")]
    PoolAllocationFailed,
    #[error("A buffer pool cannot be created with the specified attributes.")]
    InvalidPoolAttributes,
    #[error("A scan hasn't completely traversed the CVBufferPool due to a concurrent operation.)")]
    Retry,
    #[error("Could not get base address of cv pixel buffer")]
    BaseAddress,
    #[error("Could not lock base address of cv pixel buffer")]
    Lock,
    #[error("Could not unlock base address of cv pixel buffer")]
    Unlock,
    #[error("An unknown error occurred with code {0}")]
    UnknownError(TUnknown),
}

impl From<OSStatus> for CVPixelBufferError {
    fn from(value: OSStatus) -> Self {
        match value {
            INVALID_ARGUMENT => CVPixelBufferError::InvalidArgument,
            ALLOCATION_FAILED => CVPixelBufferError::AllocationFailed,
            UNSUPPORTED => CVPixelBufferError::Unsupported,
            INVALID_DISPLAY => CVPixelBufferError::InvalidDisplay,
            DISPLAYLINK_ALREADY_RUNNING => CVPixelBufferError::DisplayLinkAlreadyRunning,
            DISPLAYLINK_NOT_RUNNING => CVPixelBufferError::DisplayLinkNotRunning,
            DISPLAYLINK_CALLBACKS_NOT_SET => CVPixelBufferError::DisplayLinkCallbacksNotSet,
            INVALID_PIXEL_FORMAT => CVPixelBufferError::InvalidPixelFormat,
            INVALID_SIZE => CVPixelBufferError::InvalidSize,
            INVALID_PIXEL_BUFFER_ATTRIBUTES => CVPixelBufferError::InvalidPixelBufferAttributes,
            PIXEL_BUFFER_NOT_OPENGL_COMPATIBLE => {
                CVPixelBufferError::PixelBufferNotOpenGLCompatible
            }
            PIXEL_BUFFER_NOT_METAL_COMPATIBLE => CVPixelBufferError::PixelBufferNotMetalCompatible,
            WOULD_EXCEED_ALLOCATION_THRESHOLD => CVPixelBufferError::WouldExceedAllocationThreshold,
            POOL_ALLOCATION_FAILED => CVPixelBufferError::PoolAllocationFailed,
            INVALID_POOL_ATTRIBUTES => CVPixelBufferError::InvalidPoolAttributes,
            RETRY => CVPixelBufferError::Retry,
            ERROR => CVPixelBufferError::UnknownError(ERROR),
            _ => CVPixelBufferError::UnknownError(value),
        }
    }
}
impl From<CVPixelBufferError> for OSStatus {
    fn from(value: CVPixelBufferError) -> Self {
        match value {
            CVPixelBufferError::InvalidArgument => INVALID_ARGUMENT,
            CVPixelBufferError::AllocationFailed => ALLOCATION_FAILED,
            CVPixelBufferError::Unsupported => UNSUPPORTED,
            CVPixelBufferError::InvalidDisplay => INVALID_DISPLAY,
            CVPixelBufferError::DisplayLinkAlreadyRunning => DISPLAYLINK_ALREADY_RUNNING,
            CVPixelBufferError::DisplayLinkNotRunning => DISPLAYLINK_NOT_RUNNING,
            CVPixelBufferError::DisplayLinkCallbacksNotSet => DISPLAYLINK_CALLBACKS_NOT_SET,
            CVPixelBufferError::InvalidPixelFormat => INVALID_PIXEL_FORMAT,
            CVPixelBufferError::InvalidSize => INVALID_SIZE,
            CVPixelBufferError::InvalidPixelBufferAttributes => INVALID_PIXEL_BUFFER_ATTRIBUTES,
            CVPixelBufferError::PixelBufferNotOpenGLCompatible => {
                PIXEL_BUFFER_NOT_OPENGL_COMPATIBLE
            }
            CVPixelBufferError::PixelBufferNotMetalCompatible => PIXEL_BUFFER_NOT_METAL_COMPATIBLE,
            CVPixelBufferError::WouldExceedAllocationThreshold => WOULD_EXCEED_ALLOCATION_THRESHOLD,
            CVPixelBufferError::PoolAllocationFailed => POOL_ALLOCATION_FAILED,
            CVPixelBufferError::InvalidPoolAttributes => INVALID_POOL_ATTRIBUTES,
            CVPixelBufferError::Retry => RETRY,
            CVPixelBufferError::UnknownError(ERROR) => ERROR,
            _ => ERROR,
        }
    }
}
