use std::arch::wasm32::{
    i8x16_neg, i8x16_shr, u16x8_extend_high_u8x16, u16x8_extend_low_u8x16, u16x8_mul, u16x8_shr,
    u16x8_splat, u8x16_add, u8x16_eq, u8x16_shr, u8x16_shuffle, u8x16_splat, u8x16_swizzle, v128,
    v128_and, v128_or,
};

use anyhow::{anyhow, Result};

use crate::impl_v128::{u16x8_cycle, u16x8_to_array, u8x16_cycle, u8x16_load, u8x16_reduce_or};

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
    let lut_lo = u8x16_load(&[
        0x15, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x13, 0x1A, 0x1B, 0x1B, 0x1B,
        0x1A,
    ]);

    let lut_hi = u8x16_load(&[
        0x10, 0x10, 0x01, 0x02, 0x04, 0x08, 0x04, 0x08, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
        0x10,
    ]);

    let lo = u8x16_swizzle(lut_lo, v128_and(vectorized_ascii, u8x16_splat(0x0F)));
    let hi = u8x16_swizzle(lut_hi, u8x16_shr(vectorized_ascii, 4));

    !u8x16_reduce_or(v128_and(lo, hi))
}

#[inline]
pub fn decode(ascii: &[u8; 16]) -> Result<v128> {
    let vectorized_ascii = u8x16_load(ascii);
    let ascii_hashes = hash(vectorized_ascii);
    let sextets = sextets(vectorized_ascii, ascii_hashes);

    if !check_valid_characters(vectorized_ascii) {
        return Err(anyhow!("invalid ascii characters"));
    }

    let low_sextets = u16x8_extend_low_u8x16(sextets);
    let high_sextets = u16x8_extend_high_u8x16(sextets);

    let mask = u16x8_cycle(&[1 << 2, 1 << 4, 1 << 6, 1 << 8]);
    let shifted_low_sextets = u16x8_mul(low_sextets, mask);
    let shifted_high_sextets = u16x8_mul(high_sextets, mask);

    let lo = {
        let lo_low_sextets = v128_and(shifted_low_sextets, u16x8_splat(0xFF));
        let lo_high_sextets = v128_and(shifted_high_sextets, u16x8_splat(0xFF));

        let lo_low_sextets_arr: [u16; 8] = u16x8_to_array(lo_low_sextets);
        let lo_high_sextets_arr: [u16; 8] = u16x8_to_array(lo_high_sextets);

        let lo_low_sextets: [u8; 8] = lo_low_sextets_arr.map(|n| n as u8);
        let lo_high_sextets: [u8; 8] = lo_high_sextets_arr.map(|n| n as u8);

        let mut lo_combined_sextets = [0u8; 16];
        lo_combined_sextets[0..8].copy_from_slice(&lo_low_sextets);
        lo_combined_sextets[8..16].copy_from_slice(&lo_high_sextets);

        u8x16_load(&lo_combined_sextets)
    };

    let hi = {
        let hi_low_sextets = u16x8_shr(shifted_low_sextets, 8);
        let hi_high_sextets = u16x8_shr(shifted_high_sextets, 8);

        let hi_low_sextets_arr: [u16; 8] = u16x8_to_array(hi_low_sextets);
        let hi_high_sextets_arr: [u16; 8] = u16x8_to_array(hi_high_sextets);

        let hi_low_sextets: [u8; 8] = hi_low_sextets_arr.map(|n| n as u8);
        let hi_high_sextets: [u8; 8] = hi_high_sextets_arr.map(|n| n as u8);

        let mut hi_combined_sextets = [0u8; 16];
        hi_combined_sextets[0..8].copy_from_slice(&hi_low_sextets);
        hi_combined_sextets[8..16].copy_from_slice(&hi_high_sextets);

        u8x16_load(&hi_combined_sextets)
    };

    let hi = u8x16_shuffle::<1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0>(hi, hi);
    let decoded_chunks = v128_or(lo, hi);

    let output = u8x16_shuffle::<0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14, 16, 17, 18, 20>(
        decoded_chunks,
        u8x16_splat(0),
    );

    Ok(output)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;
    use crate::impl_v128::u8x16_to_array;

    #[wasm_bindgen_test]
    fn test_hashes() {
        let ascii = u8x16_load(b"AZM035+/2acz126m");
        let ascii_hashes = hash(ascii);

        assert_eq!(
            u8x16_to_array(ascii_hashes),
            [4, 5, 4, 3, 3, 3, 2, 1, 3, 6, 6, 7, 3, 3, 3, 6]
        );
    }

    #[wasm_bindgen_test]
    fn test_sextets() {
        let vectorized_ascii = u8x16_load(b"abcdefghabcdefgh");
        let ascii_hashes = hash(vectorized_ascii);
        let sextets = sextets(vectorized_ascii, ascii_hashes);

        assert_eq!(
            u8x16_to_array(sextets),
            [26, 27, 28, 29, 30, 31, 32, 33, 26, 27, 28, 29, 30, 31, 32, 33]
        )
    }

    #[wasm_bindgen_test]
    fn test_check_valid_characters() {
        for valid_ascii in [b"0123456788912345", b"abcdefghabcdefgh"].iter() {
            let vectorized_ascii = u8x16_load(valid_ascii);
            assert!(check_valid_characters(vectorized_ascii));
        }

        let vectorized_ascii = u8x16_splat(126);
        assert!(!check_valid_characters(vectorized_ascii));

        let vectorized_ascii = u8x16_splat(127);
        assert!(!check_valid_characters(vectorized_ascii));

        let vectorized_ascii = u8x16_splat(128);
        assert!(!check_valid_characters(vectorized_ascii));
    }

    #[wasm_bindgen_test]
    fn test_check_valid_characters_2() {
        let valid_base64_chars: BTreeSet<u8> = [
            b'A'..=b'Z', // Uppercase letters A-Z
            b'a'..=b'z', // Lowercase letters a-z
            b'0'..=b'9', // Digits 0-9
        ]
        .iter()
        .flat_map(|range| range.clone())
        .chain([b'+', b'/'].iter().copied())
        .collect();

        for i in 0..u8::MAX {
            assert!(match valid_base64_chars.contains(&i) {
                true => check_valid_characters(u8x16_splat(i)),
                false => !check_valid_characters(u8x16_splat(i)),
            });
        }
    }
}
