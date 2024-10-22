
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioBuffer {
    pub number_channels: u32,
    pub data_bytes_size: u32,
    data: *mut u8,
}

impl AudioBuffer {
    pub fn data(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data, self.data_bytes_size as usize) }
    }
}
