#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct CMTime {
    pub value: i64,
    pub timescale: i32,
    pub flags: u32,
    pub epoch: i64,
}
