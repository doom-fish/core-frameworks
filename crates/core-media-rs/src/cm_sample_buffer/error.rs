use core_foundation::base::OSStatus;
use thiserror::Error;

const FLAG_OFFSET: OSStatus = -12729;

const ALLOCATION_FAILED: OSStatus = FLAG_OFFSET - 1;
const REQUIRED_PARAMETER_MISSING: OSStatus = FLAG_OFFSET - 2;
const ALREADY_HAS_DATA_BUFFER: OSStatus = FLAG_OFFSET - 3;
const BUFFER_NOT_READY: OSStatus = FLAG_OFFSET - 4;
const SAMPLE_INDEX_OUT_OF_RANGE: OSStatus = FLAG_OFFSET - 5;
const BUFFER_HAS_NO_SAMPLE_SIZES: OSStatus = FLAG_OFFSET - 6;
const BUFFER_HAS_NO_SAMPLE_TIMING_INFO: OSStatus = FLAG_OFFSET - 7;
const ARRAY_TOO_SMALL: OSStatus = FLAG_OFFSET - 8;
const INVALID_ENTRY_COUNT: OSStatus = FLAG_OFFSET - 9;
const CANNOT_SUBDIVIDE: OSStatus = FLAG_OFFSET - 10;
const SAMPLE_TIMING_INFO_INVALID: OSStatus = FLAG_OFFSET - 11;
const INVALID_MEDIA_TYPE_FOR_OPERATION: OSStatus = FLAG_OFFSET - 12;
const INVALID_SAMPLE_DATA: OSStatus = FLAG_OFFSET - 13;
const INVALID_MEDIA_FORMAT: OSStatus = FLAG_OFFSET - 14;
const INVALIDATED: OSStatus = FLAG_OFFSET - 15;
const DATA_FAILED: OSStatus = FLAG_OFFSET - 16;
const DATA_CANCELED: OSStatus = FLAG_OFFSET - 17;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CMSampleBufferError<TUnknown = OSStatus> {
    #[error("The system failed to allocate memory.")]
    AllocationFailed,
    #[error("An attempt to set data on a sample buffer failed because that buffer already contains media data.")]
    AlreadyHasDataBuffer,
    #[error("The output array isn’t large enough to hold the requested array.")]
    ArrayTooSmall,
    #[error("A request for sample sizes on a buffer failed because the buffer doesn’t provide that information.")]
    BufferHasNoSampleSizes,
    #[error("A request for sample timing on a buffer failed because the buffer doesn’t contain that information.")]
    BufferHasNoSampleTimingInfo,
    #[error("The system can’t make the buffer’s data ready for use.")]
    BufferNotReady,
    #[error("A sample buffer doesn’t contain sample sizes.")]
    CannotSubdivide,
    #[error("A sample buffer canceled its data-loading operation.")]
    DataCanceled,
    #[error("A sample buffer failed to load its data.")]
    DataFailed,
    #[error("A timing or size value isn’t within the allowed range.")]
    InvalidEntryCount,
    #[error("The media format doesn’t match the sample buffer’s format description.")]
    InvalidMediaFormat,
    #[error("The media type that the format description defines isn’t a value for the requested operation.")]
    InvalidMediaTypeForOperation,
    #[error("The sample buffer contains bad data.")]
    InvalidSampleData,
    #[error("A sample buffer invalidated its data.")]
    Invalidated,
    #[error("A required parameter’s value is invalid.")]
    RequiredParameterMissing,
    #[error("The sample index is outside the range of samples that the buffer contains.")]
    SampleIndexOutOfRange,
    #[error("The sample buffer unexpectedly contains nonnumeric sample-timing information.")]
    SampleTimingInfoInvalid,
    #[error("Could not get format description from sample buffer.")]
    CouldNotGetFormatDescription,
    #[error("Could not get sample attachments from sample buffer.")]
    CouldNotGetSampleAttachments,
    #[error("Could not get data buffer from sample buffer.")]
    CouldNotGetDataBuffer,
    #[error("An unknown error occurred with code {0}")]
    UnknownError(TUnknown),
}

pub const NO_ERROR: OSStatus = 0;

impl From<OSStatus> for CMSampleBufferError {
    fn from(value: OSStatus) -> Self {
        match value {
            ALREADY_HAS_DATA_BUFFER => CMSampleBufferError::AlreadyHasDataBuffer,
            ALLOCATION_FAILED => CMSampleBufferError::AllocationFailed,
            ARRAY_TOO_SMALL => CMSampleBufferError::ArrayTooSmall,
            BUFFER_HAS_NO_SAMPLE_SIZES => CMSampleBufferError::BufferHasNoSampleSizes,
            BUFFER_HAS_NO_SAMPLE_TIMING_INFO => CMSampleBufferError::BufferHasNoSampleTimingInfo,
            BUFFER_NOT_READY => CMSampleBufferError::BufferNotReady,
            CANNOT_SUBDIVIDE => CMSampleBufferError::CannotSubdivide,
            DATA_CANCELED => CMSampleBufferError::DataCanceled,
            DATA_FAILED => CMSampleBufferError::DataFailed,
            INVALID_ENTRY_COUNT => CMSampleBufferError::InvalidEntryCount,
            INVALID_MEDIA_FORMAT => CMSampleBufferError::InvalidMediaFormat,
            INVALID_SAMPLE_DATA => CMSampleBufferError::InvalidSampleData,
            INVALID_MEDIA_TYPE_FOR_OPERATION => CMSampleBufferError::InvalidMediaTypeForOperation,
            INVALIDATED => CMSampleBufferError::Invalidated,
            REQUIRED_PARAMETER_MISSING => CMSampleBufferError::RequiredParameterMissing,
            SAMPLE_INDEX_OUT_OF_RANGE => CMSampleBufferError::SampleIndexOutOfRange,
            SAMPLE_TIMING_INFO_INVALID => CMSampleBufferError::SampleTimingInfoInvalid,

            _ => CMSampleBufferError::UnknownError(value),
        }
    }
}
impl From<CMSampleBufferError> for OSStatus {
    fn from(value: CMSampleBufferError) -> Self {
        match value {
            CMSampleBufferError::AlreadyHasDataBuffer => ALREADY_HAS_DATA_BUFFER,
            CMSampleBufferError::AllocationFailed => ALLOCATION_FAILED,
            CMSampleBufferError::ArrayTooSmall => ARRAY_TOO_SMALL,
            CMSampleBufferError::BufferHasNoSampleSizes => BUFFER_HAS_NO_SAMPLE_SIZES,
            CMSampleBufferError::BufferHasNoSampleTimingInfo => BUFFER_HAS_NO_SAMPLE_TIMING_INFO,
            CMSampleBufferError::BufferNotReady => BUFFER_NOT_READY,
            CMSampleBufferError::CannotSubdivide => CANNOT_SUBDIVIDE,
            CMSampleBufferError::DataCanceled => DATA_CANCELED,
            CMSampleBufferError::DataFailed => DATA_FAILED,
            CMSampleBufferError::InvalidEntryCount => INVALID_ENTRY_COUNT,
            CMSampleBufferError::InvalidSampleData => INVALID_SAMPLE_DATA,
            CMSampleBufferError::InvalidMediaFormat => INVALID_MEDIA_FORMAT,
            CMSampleBufferError::InvalidMediaTypeForOperation => INVALID_MEDIA_TYPE_FOR_OPERATION,
            CMSampleBufferError::Invalidated => INVALIDATED,
            CMSampleBufferError::RequiredParameterMissing => REQUIRED_PARAMETER_MISSING,
            CMSampleBufferError::SampleIndexOutOfRange => SAMPLE_INDEX_OUT_OF_RANGE,
            CMSampleBufferError::SampleTimingInfoInvalid => SAMPLE_TIMING_INFO_INVALID,
            CMSampleBufferError::CouldNotGetFormatDescription => -1,
            CMSampleBufferError::CouldNotGetSampleAttachments => -1,
            CMSampleBufferError::CouldNotGetDataBuffer => -1,
            CMSampleBufferError::UnknownError(value) => value,
        }
    }
}
