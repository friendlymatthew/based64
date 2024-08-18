pub(super) fn decoded_len(input: usize) -> usize {
    let mod4 = input % 4;
    input / 4 * 3 + (mod4 - mod4 / 2)
}

pub(super) fn encoded_len(input: usize) -> usize {
    let mod3 = input % 3;
    input / 3 * 4 + (mod3 + (mod3 + 1) / 2)
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[wasm_bindgen_test]
    fn test_decoded_len() {
        assert_eq!(12, decoded_len(16));
    }
}
