use std::{ffi::c_void, ptr::NonNull};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioValueTranslation<TInput = c_void, TOutput = c_void> {
    pub input_data: NonNull<TInput>,
    pub output_data: NonNull<TOutput>,
    pub input_data_size: u32,
    pub output_data_size: u32,
}

impl<TInput, TOutput> AudioValueTranslation<TInput, TOutput> {
    pub fn new(
        input_data: NonNull<TInput>,
        output_data: NonNull<TOutput>,
        input_data_size: u32,
        output_data_size: u32,
    ) -> Self {
        Self {
            input_data,
            output_data,
            input_data_size,
            output_data_size,
        }
    }
}
