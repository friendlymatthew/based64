use base64::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn base64_encode(data: &[u8]) -> Result<String, JsValue> {
    Ok(BASE64_STANDARD.encode(data))
}

#[wasm_bindgen]
pub fn base64_decode(ascii: &[u8]) -> Result<Vec<u8>, JsValue> {
    BASE64_STANDARD
        .decode(ascii)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[wasm_bindgen_test]
    fn test_hello_world_base64() -> Result<(), JsValue> {
        let encoded_data = b"SGVsbG8gV29ybGQ=";
        let raw_data = b"Hello World";

        let ascii = base64_encode(raw_data)?;
        assert_eq!(ascii.as_bytes(), encoded_data);

        let decoded_raw = base64_decode(ascii.as_bytes())?;
        assert_eq!(decoded_raw, raw_data);

        Ok(())
    }
}
