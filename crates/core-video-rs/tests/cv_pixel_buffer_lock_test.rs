use std::error::Error;

use core_utils_rs::four_char_code::FourCharCode;

use core_video_rs::cv_pixel_buffer::attributes::PixelBufferAttributes;
use core_video_rs::cv_pixel_buffer::lock::{LockTrait, MutLockTrait};
use core_video_rs::cv_pixel_buffer::CVPixelBuffer;
const WIDTH: u32 = 10;
const HEIGHT: u32 = 10;

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
        println!("{b:?}");
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
