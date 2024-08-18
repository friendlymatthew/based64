#![cfg(target_arch = "wasm32")]
#![feature(simd_wasm64)]

use decode::decode;

mod decode;
mod encode;
pub mod impl_v128;

#[derive(Copy, Clone, Debug)]
pub struct Error;

fn decoded_len(input: usize) -> usize {
    let mod4 = input % 4;
    input / 4 * 3 + (mod4 - mod4 / 2)
}

fn encoded_len(input: usize) -> usize {
    let mod3 = input % 3;
    input / 3 * 4 + (mod3 + (mod3 + 1) / 2)
}

pub fn decode_to(data: &[u8], out: &mut Vec<u8>) -> Result<(), Error> {
    let n = data.len();
    assert_eq!(n % 4, 0);
    let data = match data {
        [p @ .., b'=', b'='] | [p @ .., b'='] | p => p,
    };

    if data.is_empty() {
        return Ok(());
    }

    out.reserve(decoded_len(data.len()) + n);
    let mut _raw_out = out.as_mut_ptr_range().end;
    let mut chunks = data.chunks_exact(16);
    let mut failed = false;

    for chunk in &mut chunks {
        let ascii = chunk.try_into().expect("Slice with incorrect length");
        let decoded = decode(ascii);
        failed |= decoded.is_err();
    }

    let rest = chunks.remainder();
    if !rest.is_empty() {
        let mut ascii = [0u8; 16];
        ascii.copy_from_slice(rest);
        let decoded = decode(&ascii);
        failed |= decoded.is_err();
    }

    if failed {
        return Err(Error);
    }

    unsafe {
        let new_len = _raw_out.offset_from(out.as_ptr());
        out.set_len(new_len as usize);
    }

    Ok(())
}
