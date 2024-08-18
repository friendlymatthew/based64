use std::arch::wasm32::{v128, v128_load, v128_store};

/// given a u8x16, returns the cumulative bitwise "or"
/// across elements of a vector
#[inline]
pub fn u8x16_reduce_or(val: v128) -> bool {
    let arr = u8x16_to_byte_array(val);
    let mut res = arr[0] != 0;
    for &byte in &arr[1..] {
        res |= byte != 0;
    }
    res
}

#[inline]
pub fn u8x16_to_byte_array(val: v128) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    unsafe {
        std::ptr::copy_nonoverlapping(&val as *const v128 as *const u8, bytes.as_mut_ptr(), 16);
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
pub fn load_u16x8(u16x8: &[u16; 8]) -> v128 {
    unsafe { v128_load(u16x8.as_ptr() as *const v128) }
}

#[inline]
pub fn _load_i8x16(i8x16: &[i8; 16]) -> v128 {
    unsafe { v128_load(i8x16.as_ptr() as *const v128) }
}

#[inline]
pub fn u8x16_cycle(pattern: &[u8]) -> v128 {
    let mut out = [pattern[0]; 16];
    let mut i = 0;

    while i < 16 {
        out[i] = pattern[i % pattern.len()];
        i += 1;
    }

    load_u8x16(&out)
}

#[inline]
pub fn u16x8_cycle(pattern: &[u16]) -> v128 {
    let mut out = [pattern[0]; 8];
    let mut i = 0;

    while i < 8 {
        out[i] = pattern[i % pattern.len()];
        i += 1;
    }

    load_u16x8(&out)
}

#[cfg(test)]
mod tests {
    use std::arch::wasm32::u8x16_splat;

    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[wasm_bindgen_test]
    fn test_cycle_identity() {
        let cycled = u8x16_cycle(&[!0, 16, 19, 4, 191, 191, 185, 185]);

        assert_eq!(
            u8x16_to_byte_array(cycled),
            [!0, 16, 19, 4, 191, 191, 185, 185, !0, 16, 19, 4, 191, 191, 185, 185]
        );
    }

    #[wasm_bindgen_test]
    fn test_cycle_identity_2() {
        let cycled_2 = u8x16_cycle(&[1, 2, 3]);
        assert_eq!(
            u8x16_to_byte_array(cycled_2),
            [1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1]
        );
    }

    #[wasm_bindgen_test]
    fn test_u8x16_reduce_or() {
        let res = u8x16_reduce_or(u8x16_splat(0));

        assert!(!res);

        let res = u8x16_reduce_or(u8x16_cycle(&[0, 1]));
        assert!(res);
    }
}
