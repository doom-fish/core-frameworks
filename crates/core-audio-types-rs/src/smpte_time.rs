use crate::{smpte_time_flags::SMTPETimeFlags, smpte_time_type::SMPTETimeType};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SMTPETime {
    pub counter: u32,
    pub flags: SMTPETimeFlags,
    pub frames: i16,
    pub hours: i16,
    pub minutes: i16,
    pub seconds: i16,
    pub subframe_divisor: i16,
    pub subframes: i16,
    pub time_type: SMPTETimeType,
}

impl SMTPETime {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        counter: u32,
        flags: SMTPETimeFlags,
        frames: i16,
        hours: i16,
        minutes: i16,
        seconds: i16,
        subframe_divisor: i16,
        subframes: i16,
        time_type: SMPTETimeType,
    ) -> Self {
        Self {
            counter,
            flags,
            frames,
            hours,
            minutes,
            seconds,
            subframe_divisor,
            subframes,
            time_type,
        }
    }
}
