use std::arch::wasm32::{v128, v128_bitselect, v128_load};

use paste::paste;

macro_rules! impl_v128 {
    ($ty:ty, $lane_count:expr) => {
        paste! {

            /// [< $ty x $lane_count _load>] loads data into a 128-bit vector.
            #[inline]
            pub fn [< $ty x $lane_count _load>](data: &[$ty; $lane_count]) -> v128 {
                 unsafe { v128_load(data.as_ptr() as *const v128) }
            }


            /// `[< $ty x $lane_count  _to_array>]` copies contents of the
            /// 128-bit vector into a array with the respective type and lane size.
            #[inline]
            pub fn [< $ty x $lane_count  _to_array>](val: v128) -> [$ty; $lane_count] {
                let mut buf = [0 as $ty; $lane_count];
                unsafe {
                    std::ptr::copy_nonoverlapping(&val as *const v128 as *const $ty, buf.as_mut_ptr(), $lane_count);
                }
                buf
            }

            /// `[< $ty x $lane_count _reduce_or>]` returns the cumulative
            /// bitwise || across the elements of the vector.
            #[inline]
            pub fn [< $ty x $lane_count _reduce_or>](val: v128) -> bool {
                // todo: this is just cumulative ||
                let arr = [< $ty x $lane_count _to_array>](val);
                let mut res = arr[0] != 0;
                for &byte in &arr[1..] {
                    res |= byte != 0;
                }
                res
            }

            /// `[<$ty x $lane_count _cycle>]` repeatedly
            /// applies the pattern provided till the vector is full.
            #[inline]
            pub fn [<$ty x $lane_count _cycle>](pattern: &[$ty]) -> v128 {
                let mut out = [pattern[0]; $lane_count];
                let mut i = 0;

                while i < $lane_count {
                    out[i] = pattern[i % pattern.len()];
                    i += 1;
                }

                [<$ty x $lane_count _load>](&out)
            }

            use std::arch::wasm32::[<$ty x $lane_count _splat>];

            /// `[<$ty x $lane_count _mask_splat>]` is based on the bitmask,
            ///       where true => use `select_if_true`
            ///             false => 0
            #[inline]
            pub fn [<$ty x $lane_count _mask_splat>](mask: v128, select_if_true: $ty) -> v128 {
                let splat_val = [<$ty x $lane_count _splat>](select_if_true);
                let default_val = [<$ty x $lane_count _splat>](0 as $ty);
                v128_bitselect(splat_val, default_val, mask)
            }

        }
    };
}

impl_v128!(u8, 16);
impl_v128!(i8, 16);
impl_v128!(u16, 8);
impl_v128!(i16, 8);

#[cfg(test)]
mod tests {
    use std::arch::wasm32::{i16x8_splat, u8x16_splat, v128_not};

    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;
    use crate::impl_v128::{u8x16_cycle, u8x16_reduce_or};

    #[wasm_bindgen_test]
    fn test_cycle_identity() {
        let cycled = u8x16_cycle(&[!0, 16, 19, 4, 191, 191, 185, 185]);

        assert_eq!(
            u8x16_to_array(cycled),
            [!0, 16, 19, 4, 191, 191, 185, 185, !0, 16, 19, 4, 191, 191, 185, 185]
        );
    }

    #[wasm_bindgen_test]
    fn test_cycle_identity_2() {
        let cycled_2 = i16x8_cycle(&[1, 2, 3]);
        assert_eq!(i16x8_to_array(cycled_2), [1, 2, 3, 1, 2, 3, 1, 2,]);
    }

    #[wasm_bindgen_test]
    fn test_u8x16_reduce_or() {
        let res = u8x16_reduce_or(u8x16_splat(0));

        assert!(!res);

        let res = u8x16_reduce_or(u8x16_cycle(&[0, 1]));
        assert!(res);
    }

    #[wasm_bindgen_test]
    fn test_i16x8_reduce_or() {
        let res = i16x8_reduce_or(i16x8_splat(0));
        assert!(!res);

        let res = i16x8_reduce_or(i16x8_load(&[0, 0, 0, 0, 0, 0, 0, 1]));
        assert!(res);
    }

    #[wasm_bindgen_test]
    fn test_u16x8_mask_splat() {
        let bitmask = u16x8_load(&[0, 0xFFFF, 0, 0xFFFF, 0, 0xFFFF, 0, 0xFFFF]);
        let res = u16x8_mask_splat(bitmask, 67);

        assert_eq!(u16x8_to_array(res), [0, 67, 0, 67, 0, 67, 0, 67]);
    }

    #[wasm_bindgen_test]
    fn test_v128_not() {
        let res = i8x16_cycle(&[1, 2, 3, 0]);
        let res = v128_not(res);

        assert_eq!(i8x16_to_array(res)[0..4], [-2, -3, -4, -1])
    }
}
