#[cfg(test)]
mod tests {
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{decode, encode};

    fn valid_chars() -> Vec<char> {
        [
            'A'..='Z', // Uppercase letters A-Z
            'a'..='z', // Lowercase letters a-z
            '0'..='9', // Digits 0-9
        ]
        .iter()
        .flat_map(|range| range.clone())
        .chain(['+', '/'].iter().copied())
        .collect()
    }

    #[inline]
    fn xor_shift(seed: &mut u32) -> u32 {
        let mut x = *seed;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        *seed = x;
        x
    }

    #[wasm_bindgen_test]
    fn fuzz() -> Result<(), JsValue> {
        let valid_characters = valid_chars();
        let mut seed = 42;

        for i in 0..1000 {
            let blob_length = xor_shift(&mut seed) % 100 + 100;
            let mut random_word = String::with_capacity(blob_length as usize);

            for _ in 0..blob_length {
                let idx = xor_shift(&mut seed) % valid_characters.len() as u32;
                let rand_char = valid_characters[idx as usize];
                random_word.push(rand_char);
            }

            let ascii = encode(random_word.as_bytes())?;
            let decoded = decode(&ascii)?;

            assert_eq!(
                decoded,
                random_word.as_bytes(),
                "failed at iter: {i}\nencoded_ascii: {:?}\nrandom_ascii: {:?}\n",
                String::from_utf8(decoded.clone()),
                random_word.clone()
            );
        }

        Ok(())
    }
}
