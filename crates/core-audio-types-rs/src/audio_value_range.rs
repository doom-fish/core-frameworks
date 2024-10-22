#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioValueRange {
    pub minimum: f64,
    pub maximum: f64,
}

impl AudioValueRange {
    pub fn new(minimum: f64, maximum: f64) -> Self {
        Self { minimum, maximum }
    }
}
