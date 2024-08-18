use std::arch::wasm32::{
    i16x8_extend_high_u8x16, u16x8_add, u16x8_extend_low_u8x16, u16x8_shl, u8x16_ge, u8x16_shr,
    u8x16_shuffle, u8x16_splat, u8x16_sub, u8x16_sub_sat, u8x16_swizzle, v128, v128_and, v128_or,
};

use anyhow::Result;

use crate::impl_v128::{u16x8_to_array, u8x16_cycle, u8x16_load, u8x16_mask_splat};

pub(super) fn encode(data: &[u8; 16]) -> Result<v128> {
    let data = u8x16_load(data);
    let data =
        u8x16_shuffle::<0, 1, 2, 16, 3, 4, 5, 16, 6, 7, 8, 16, 9, 10, 11, 16>(data, u8x16_splat(0));

    let mask = u8x16_cycle(&[0b11111100, 0b11110000, 0b11000000, 0b00000000]);
    let mask_not = u8x16_cycle(&[3, 15, 63, 255]);

    let lo = v128_and(data, mask);
    let hi = v128_and(data, mask_not);

    let hi = u8x16_shuffle::<15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14>(hi, hi);

    let (lo_low, lo_high) = (u16x8_extend_low_u8x16(lo), i16x8_extend_high_u8x16(lo));
    let (hi_low, hi_high) = {
        let (hi_low, hi_high) = (u16x8_extend_low_u8x16(hi), i16x8_extend_high_u8x16(hi));
        (u16x8_shl(hi_low, 8), u16x8_shl(hi_high, 8))
    };

    let (lo_shifted, hi_shifted) = (v128_or(lo_low, hi_low), v128_or(lo_high, hi_high));

    let sextets = {
        // todo: division is hard because it requires newton-rhapson’s method
        // shifted >> u16x8_cycle(&[2, 4, 6, 8])
        let pattern = [2, 4, 6, 8];

        let (lo_shifted_arr, hi_shifted_arr) =
            (u16x8_to_array(lo_shifted), u16x8_to_array(hi_shifted));

        let mut sextets = [0u8; 16];

        for i in 0..lo_shifted_arr.len() {
            sextets[i] = (lo_shifted_arr[i] >> pattern[i % pattern.len()]) as u8;
            sextets[i + lo_shifted_arr.len()] =
                (hi_shifted_arr[i] >> pattern[i % pattern.len()]) as u8;
        }

        u8x16_load(&sextets)
    };

    let hashes = u8x16_sub_sat(sextets, u8x16_splat(0x0A));

    let bitmask_1 = u8x16_ge(sextets, u8x16_splat(0x34));
    let mask_splat_1 = u8x16_mask_splat(bitmask_1, 0x0f);

    let bitmask_2 = u8x16_ge(sextets, u8x16_splat(0x3e));
    let mask_splat_2 = u8x16_mask_splat(bitmask_2, 0x1c);

    let hashes = u8x16_shr(u16x8_add(u16x8_add(hashes, mask_splat_1), mask_splat_2), 4);
    let offsets = u8x16_swizzle(u8x16_cycle(&[191, 185, 185, 4, 4, 19, 16, !0]), hashes);

    Ok(u8x16_sub(sextets, offsets))
}
