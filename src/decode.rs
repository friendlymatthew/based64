use crate::v128::load_u8x16;
use std::arch::wasm::*;

#[inline]
fn compute_hash(ascii: v128) -> v128 {
    let shifted = i8x16_shr(ascii, 4);
    let mask = u8x16_eq(ascii, u8x16_splat(b'/'));
    let norm_mask = v128_and(mask, u8x16_splat(1));
    let neg_norm_mask = i8x16_neg(norm_mask);

    u8x16_add(shifted, neg_norm_mask)
}

#[inline]
pub fn decode(ascii: &[u8; 16]) {
    let vectorized_ascii = load_u8x16(ascii);
    let ascii_hashes = compute_hash(vectorized_ascii);

    let sextets = u8x16_add(
        vectorized_ascii,
        u8x16_swizzle(
            load_u8x16(&[
                !0, 16, 19, 4, 191, 191, 185, 185, !0, 16, 19, 4, 191, 191, 185, 185,
            ]),
            ascii_hashes,
        ),
    );

    let (lo_lut, hi_lut) = (
        load_u8x16(&[
            0b10101, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001,
            0b10001, 0b10011, 0b11010, 0b11011, 0b11011, 0b11011, 0b11010,
        ]),
        load_u8x16(&[
            0b10000, 0b10000, 0b00001, 0b00010, 0b00100, 0b01000, 0b00100, 0b01000, 0b10000,
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000,
        ]),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v128::v128_to_u8x16;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_hashes() {
        let ascii: [u8; 16] = [
            b'A', b'Z', b'M', b'0', b'3', b'5', b'+', b'/', b'2', b'a', b'c', b'z', b'1', b'2',
            b'6', b'm',
        ];

        let ascii = load_u8x16(&ascii);
        let ascii_hashes = compute_hash(ascii);

        println!("{:?}", ascii_hashes);

        assert_eq!(
            v128_to_u8x16(ascii_hashes),
            [4, 5, 4, 3, 3, 3, 2, 1, 3, 6, 6, 7, 3, 3, 3, 6]
        );
    }
}
