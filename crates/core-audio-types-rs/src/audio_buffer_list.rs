use crate::audio_buffer::AudioBuffer;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioBufferList<const SIZE: usize = 8> {
    number_buffers: usize,
    buffers: [AudioBuffer; SIZE],
}

impl AudioBufferList {
    pub fn num_buffers(&self) -> usize {
        self.number_buffers
    }
    pub fn buffers(&self) -> &[AudioBuffer] {
        &self.buffers[..self.number_buffers]
    }
    pub fn get(&self, index: usize) -> Option<&AudioBuffer> {
        self.buffers().get(index)
    }
}
