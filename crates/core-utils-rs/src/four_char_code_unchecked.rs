use four_char_code::FourCharCode;

const fn from_bytes_unchecked(mut bytes: [u8; 4]) -> FourCharCode {
    let mut null_streak = true;

    let mut i = 3usize;
    loop {
        let mut c = bytes[i];
        if c == 0 {
            if null_streak {
                c = 0x20;
                bytes[i] = c;
            } else {
                panic!("Invalid FourCharCode");
            }
        } else {
            null_streak = false;
        }

        if c <= b'\x1f' || c >= b'\x7f' {
            panic!("Invalid FourCharCode");
        }

        if i == 0 {
            break;
        }
        i -= 1;
    }

    unsafe { FourCharCode::new_unchecked(u32::from_be_bytes(bytes)) }
}
/// Returns a [FourCharCode] if slice is valid, an error describing the problem otherwise.
pub const fn from_slice_unchecked(value: &[u8]) -> FourCharCode {
    if value.len() < 4 || value.len() > 4 {
        panic!("Invalid FourCharCode");
    }

    from_bytes_unchecked([value[0], value[1], value[2], value[3]])
}

/// Returns a [FourCharCode] if string is valid, an error describing the problem otherwise.
#[allow(clippy::should_implement_trait)]
pub const fn from_str_unchecked(value: &str) -> FourCharCode {
    from_slice_unchecked(value.as_bytes())
}
