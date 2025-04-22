use crate::audio_buffer::AudioBuffer;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioBufferList<const SIZE: usize = 8> {
    number_buffers: u32,
    buffers: [AudioBuffer; SIZE],
}

impl AudioBufferList {
    pub fn num_buffers(&self) -> usize {
        self.number_buffers as usize
    }
    pub fn buffers(&self) -> &[AudioBuffer] {
        &self.buffers[..self.number_buffers as usize]
    }
    pub fn get(&self, index: usize) -> Option<&AudioBuffer> {
        self.buffers().get(index)
    }
}
