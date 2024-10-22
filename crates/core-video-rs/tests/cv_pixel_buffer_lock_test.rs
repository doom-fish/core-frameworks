use std::error::Error;

use core_utils_rs::four_char_code::FourCharCode;
use core_utils_rs::lock::{LockTrait, MutLockTrait};

use core_video_rs::cv_pixel_buffer::CVPixelBuffer;
use core_video_rs::cv_pixel_buffer::attributes::PixelBufferAttributes;
const WIDTH: usize = 10;
const HEIGHT: usize = 10;

#[test]
fn test_create_lock_write() -> Result<(), Box<dyn Error>> {
    let mut pixel_buffer = CVPixelBuffer::create(
        WIDTH,
        HEIGHT,
        FourCharCode::from_str("BGRA").unwrap(),
        PixelBufferAttributes::default(),
    )?;

    {
        let mut b = pixel_buffer.lock_mut()?;
        b.fill(123);
    };
    {
        let b = pixel_buffer.lock()?;
        for i in 1..b.len() {
            assert_eq!(b[i], 123);
        }
    };
    Ok(())
}
