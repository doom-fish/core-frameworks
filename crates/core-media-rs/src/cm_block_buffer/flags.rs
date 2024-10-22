pub type CMBlockBufferFlags = u32;

pub const NO_FLAGS: CMBlockBufferFlags = 0;
pub const ASSURE_MEMORY_NOW_FLAG: CMBlockBufferFlags = 1 << 0;
pub const ALWAYS_COPY_DATA_FLAG: CMBlockBufferFlags = 1 << 1;
pub const DONT_OPTIMIZE_DEPTH_FLAG: CMBlockBufferFlags = 1 << 2;
pub const PERMIT_EMPTY_REFERENCE_FLAG: CMBlockBufferFlags = 1 << 3;
