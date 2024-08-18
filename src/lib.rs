#![cfg(target_arch = "wasm32")]
#![feature(simd_wasm64)]

use std::arch::wasm32::v128;
use std::slice::from_raw_parts;

use anyhow::anyhow;
use common::{decoded_len, encoded_len};
use decode::decode;
use encode::encode;

mod common;
mod decode;
mod encode;
pub mod impl_v128;
use anyhow::Result;

pub fn encode_to(data: &[u8], out: &mut Vec<u8>) -> Result<()> {
    if data.is_empty() {
        return Err(anyhow!("empty data!"));
    }

    out.reserve(encoded_len(data.len()) + 16);
    let mut raw_out = out.as_mut_ptr_range().end;

    let mut start = data.as_ptr();
    let end = unsafe {
        if data.len() % 12 >= 4 {
            start.add(data.len() - data.len() % 12)
        } else if data.len() < 16 {
            start
        } else {
            start.add(data.len() - data.len() % 12 - 12)
        }
    };

    while start != end {
        let chunk = unsafe { from_raw_parts(start, 16) };
        let chunk: &[u8; 16] = chunk.try_into().expect("Slice with incorrect length");
        let encoded = encode(chunk)?;

        unsafe {
            start = start.add(12);

            raw_out.cast::<v128>().write_unaligned(encoded);
            raw_out = raw_out.add(16);
        }
    }

    let end = data.as_ptr_range().end;
    while start < end {
        let chunk = unsafe {
            let rest = end.offset_from(start) as usize;
            std::slice::from_raw_parts(start, rest.min(12))
        };

        let mut temp_chunk = [0u8; 16];
        temp_chunk[0..chunk.len()].copy_from_slice(chunk);

        let encoded = encode(&temp_chunk)?;

        unsafe {
            start = start.add(chunk.len());

            raw_out.cast::<v128>().write_unaligned(encoded);
            raw_out = raw_out.add(encoded_len(chunk.len()));
        }
    }

    unsafe {
        let new_len = raw_out.offset_from(out.as_ptr());
        out.set_len(new_len as usize);
    }

    match out.len() % 4 {
        2 => out.extend_from_slice(b"=="),
        3 => out.extend_from_slice(b"="),
        _ => {}
    }

    Ok(())
}

pub fn decode_to(data: &[u8], out: &mut Vec<u8>) -> Result<()> {
    let data = match data {
        [p @ .., b'=', b'='] | [p @ .., b'='] | p => p,
    };

    if data.is_empty() {
        return Ok(());
    }

    out.reserve(decoded_len(data.len()) + 16);
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
        let encoded_data = b"SGVsbG8gV29ybGQ=";
        let raw_data = b"Hello World";

        let mut out = Vec::new();
        decode_to(encoded_data, &mut out)?;
        assert_eq!(out, raw_data);

        out = Vec::new();
        encode_to(raw_data, &mut out)?;
        assert_eq!(out, encoded_data);
        Ok(())
    }
}
