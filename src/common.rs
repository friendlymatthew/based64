use std::mem::MaybeUninit;

#[derive(Copy, Clone, Debug)]
pub struct Error;

pub(super) fn decoded_len(input: usize) -> usize {
    let mod4 = input % 4;
    input / 4 * 3 + (mod4 - mod4 / 2)
}

fn encoded_len(input: usize) -> usize {
    let mod3 = input % 3;
    input / 3 * 4 + (mod3 + (mod3 + 1) / 2)
}

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
