#![cfg(target_arch = "wasm32")]
#![feature(simd_wasm64)]

use std::arch::wasm32::v128;

use anyhow::anyhow;
use common::decoded_len;
use decode::decode;

mod common;
mod decode;
pub mod impl_v128;
use anyhow::Result;

pub fn decode_to(data: &[u8], out: &mut Vec<u8>) -> Result<()> {
    let n = data.len();
    assert_eq!(n % 4, 0);
    let data = match data {
        [p @ .., b'=', b'='] | [p @ .., b'='] | p => p,
    };

    if data.is_empty() {
        return Ok(());
    }

    out.reserve(decoded_len(data.len()) + n);
    let mut raw_out = out.as_mut_ptr_range().end;

    let mut chunks = data.chunks_exact(16);
    let mut failed = false;

    for chunk in &mut chunks {
        let ascii = chunk.try_into().expect("Slice with incorrect length");
        let decoded = decode(ascii);
        failed |= decoded.is_err();
        let decoded = decoded.unwrap();

        unsafe {
            raw_out.cast::<v128>().write_unaligned(decoded);
            raw_out = raw_out.add(12);
        }
    }

    let rest = chunks.remainder();
    if !rest.is_empty() {
        let mut ascii = [b'A'; 16];
        ascii[0..rest.len()].copy_from_slice(rest);
        let decoded = decode(&ascii);
        failed |= decoded.is_err();
        let decoded = decoded.unwrap();

        unsafe {
            raw_out.cast::<v128>().write_unaligned(decoded);
            raw_out = raw_out.add(decoded_len(rest.len()));
        }
    }

    if failed {
        return Err(anyhow!("the decoding process failed unexpectedly"));
    }

    unsafe {
        let new_len = raw_out.offset_from(out.as_ptr());
        out.set_len(new_len as usize);
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[wasm_bindgen_test]
    fn test_hello_world() -> Result<()> {
        let data = b"SGVsbG8gV29ybGQ=";
        let mut out = Vec::new();
        decode_to(data, &mut out)?;
        assert_eq!(out, b"Hello World");
        Ok(())
    }
}
