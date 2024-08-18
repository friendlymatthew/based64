use std::arch::wasm32::{
    i8x16_neg, i8x16_shr, u16x8_extend_high_u8x16, u16x8_extend_low_u8x16, u8x16_add, u8x16_eq,
    u8x16_shr, u8x16_splat, u8x16_swizzle, v128, v128_and,
};

use crate::v128::{load_u8x16, u8x16_cycle, u8x16_reduce_or};
use crate::Error;

#[inline]
fn hash(ascii: v128) -> v128 {
    let shifted = i8x16_shr(ascii, 4);
    let mask = u8x16_eq(ascii, u8x16_splat(b'/'));
    let norm_mask = v128_and(mask, u8x16_splat(1));
    let neg_norm_mask = i8x16_neg(norm_mask);

    u8x16_add(shifted, neg_norm_mask)
}

#[inline]
fn sextets(vectorized_ascii: v128, ascii_hashes: v128) -> v128 {
    u8x16_add(
        vectorized_ascii,
        u8x16_swizzle(
            u8x16_cycle(&[!0, 16, 19, 4, 191, 191, 185, 185]),
            ascii_hashes,
        ),
    )
}

#[inline]
fn check_valid_characters(vectorized_ascii: v128) -> bool {
    let lut_lo = load_u8x16(&[
        0x15, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x13, 0x1A, 0x1B, 0x1B, 0x1B,
        0x1A,
    ]);

    let lut_hi = load_u8x16(&[
        0x10, 0x10, 0x01, 0x02, 0x04, 0x08, 0x04, 0x08, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
        0x10,
    ]);

    let lo = u8x16_swizzle(lut_lo, v128_and(vectorized_ascii, u8x16_splat(0x0F)));
    let hi = u8x16_swizzle(lut_hi, u8x16_shr(vectorized_ascii, 4));

    !u8x16_reduce_or(v128_and(lo, hi))
}

#[inline]
pub fn decode(ascii: &[u8; 16]) -> Result<v128, Error> {
    let vectorized_ascii = load_u8x16(ascii);
    let ascii_hashes = hash(vectorized_ascii);
    let sextets = sextets(vectorized_ascii, ascii_hashes);

    let ok = check_valid_characters(vectorized_ascii);

    if !ok {
        return Err(Error);
    }

    let low_sextets = u16x8_extend_low_u8x16(sextets);
    let high_sextets = u16x8_extend_high_u8x16(sextets);

    for &sextet in &[low_sextets, high_sextets] {
        // TODO sextet << [2, 4, 6, 8, 2, 4, 6, 8]
    }

    Ok(load_u8x16(b"1234567890123456"))
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;
    use crate::v128::u8x16_to_byte_array;

    #[wasm_bindgen_test]
    fn test_hashes() {
        let ascii = load_u8x16(b"AZM035+/2acz126m");
        let ascii_hashes = hash(ascii);

        assert_eq!(
            u8x16_to_byte_array(ascii_hashes),
            [4, 5, 4, 3, 3, 3, 2, 1, 3, 6, 6, 7, 3, 3, 3, 6]
        );
    }

    #[wasm_bindgen_test]
    fn test_sextets() {
        let vectorized_ascii = load_u8x16(b"abcdefghabcdefgh");
        let ascii_hashes = hash(vectorized_ascii);
        let sextets = sextets(vectorized_ascii, ascii_hashes);

        assert_eq!(
            u8x16_to_byte_array(sextets),
            [26, 27, 28, 29, 30, 31, 32, 33, 26, 27, 28, 29, 30, 31, 32, 33]
        )
    }

    #[wasm_bindgen_test]
    fn test_check_valid_characters() {
        for valid_ascii in [b"0123456788912345", b"abcdefghabcdefgh"].iter() {
            let vectorized_ascii = load_u8x16(valid_ascii);
            assert!(check_valid_characters(vectorized_ascii));
        }

        let vectorized_ascii = u8x16_splat(126);
        assert!(!check_valid_characters(vectorized_ascii));

        let vectorized_ascii = u8x16_splat(127);
        assert!(!check_valid_characters(vectorized_ascii));

        let vectorized_ascii = u8x16_splat(128);
        assert!(!check_valid_characters(vectorized_ascii));
    }
}
