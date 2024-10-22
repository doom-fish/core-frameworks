use std::error::Error;

use core_foundation::base::{kCFAllocatorDefault, TCFType};
use core_media_rs::{
    cm_block_buffer::{flags::NO_FLAGS, CMBlockBuffer},
    cm_format_description::{media_type::CMMediaType, CMFormatDescription},
    cm_sample_buffer::{error::CMSampleBufferError, CMSampleBuffer},
};
use core_utils_rs::four_char_code::FourCharCode;

#[test]
fn test_create() -> Result<(), Box<dyn Error>> {
    let sample_count = 0;
    let sample_timings = vec![];
    let sample_sizes = vec![];
    let allocator = unsafe { kCFAllocatorDefault };
    let blockbuf = CMBlockBuffer::create_empty(allocator, 0, NO_FLAGS)?;
    let format_description = CMFormatDescription::create(
        allocator,
        CMMediaType::VIDEO,
        FourCharCode::from_str("0000").expect("should work"),
        None,
    )?;
    let buf = {
        let v = vec![1, 2, 3];
        let buf = CMSampleBuffer::create(
            allocator,
            &blockbuf,
            false,
            move |_a| {
                let b = &v;
                println!("{b:?}");
                Err(CMSampleBufferError::UnknownError(1337))
            },
            &format_description,
            sample_count,
            &sample_timings,
            &sample_sizes,
        )?;
        println!("{:p}", buf.as_concrete_TypeRef());
        buf
    };
    if let Err(s) = buf.make_data_ready() {
        assert_eq!(s, CMSampleBufferError::UnknownError(1337));
    } else {
        unreachable!("Should not end up here!");
    }
    Ok(())
}
