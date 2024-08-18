use std::arch::wasm32::{u8x16_swizzle, v128_and};
use std::mem::MaybeUninit;

use crate::v128::{load_u8x16, u8x16_cycle};
use crate::Error;

#[inline(always)]
pub fn invert_index(array: Array) -> [u8; 16] {
    let array = Array::into_initialized(array);
    let mut out = [16; 16];
    let mut i = 0;
    while i < 16 {
        if array[i] < 16 {
            out[array[i]] = i as u8;
        }
        i += 1;
    }

    out
}

pub struct Array(pub(crate) [MaybeUninit<usize>; 16]);
impl Array {
    pub fn new<F>(mut f: F) -> Self
    where
        F: FnMut(usize) -> usize,
    {
        let mut array = [MaybeUninit::uninit(); 16];
        for (i, item) in array.iter_mut().enumerate() {
            *item = MaybeUninit::new(f(i));
        }

        Self(array)
    }

    pub fn into_initialized(self) -> [usize; 16] {
        unsafe { std::mem::transmute(self.0) }
    }
}

#[inline]
pub fn encode(data: [u8; 16]) -> Result<(), Error> {
    let table = Array::new(|idx| idx + idx / 3);
    let data = u8x16_swizzle(load_u8x16(&data), load_u8x16(&invert_index(table)));

    Ok(())
}
