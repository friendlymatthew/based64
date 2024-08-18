#![cfg(target_arch = "wasm32")]
#![feature(simd_wasm64)]

pub mod decode;
pub mod encode_wasm32;
mod v128;
