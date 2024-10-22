#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioStreamPacketDescription {
    pub start_offset: u64,
    pub variable_frames_in_packet: u32,
    pub data_byte_size: u32,
}

impl AudioStreamPacketDescription {
    pub fn new(start_offset: u64, variable_frames_in_packet: u32, data_byte_size: u32) -> Self {
        Self {
            start_offset,
            variable_frames_in_packet,
            data_byte_size,
        }
    }
}
