use std::arch::wasm::{v128, v128_load, v128_store};

#[inline]
pub fn v128_to_u8x16(val: v128) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    unsafe {
        v128_store(bytes.as_mut_ptr() as *mut v128, val);
    }
    bytes
}

#[inline]
pub fn _v128_to_i8x16(val: v128) -> [i8; 16] {
    let mut bytes = [0i8; 16];
    unsafe {
        v128_store(bytes.as_mut_ptr() as *mut v128, val);
    }
    bytes
}

#[inline]
pub fn load_u8x16(u8x16: &[u8; 16]) -> v128 {
    unsafe { v128_load(u8x16.as_ptr() as *const v128) }
}

#[inline]
pub fn _load_i8x16(i8x16: &[i8; 16]) -> v128 {
    unsafe { v128_load(i8x16.as_ptr() as *const v128) }
}
