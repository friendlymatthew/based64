# based64

A base64 codec using wasm32 SIMD intrinsics.

```rust
use based64::{decode, encode};
use wasm_bindgen::{wasm_bindgen, JsValue};

#[wasm_bindgen]
fn main() -> Result<(), JsValue> {
  let ascii = b"VGhlIGRvZyBsaWNrZWQgdGhlIG9pbCwgYW5kIGV2ZXJ5Ym9keSBsYXVnaGVkLg==";
  let message = decode(ascii)?; // The dog licked the oil, and everybody laughed.

  let encoded_to_ascii = encode(&message)?;
  assert_eq!(encoded_to_ascii, ascii.to_vec());

  Ok(())
}
```

or
```typescript
import init, {
    encode,
    decode,
} from "./pkg/based64.js";

async function run(): boolean {
    await init();
    
    let data = "howdy";
    let bytes = new TextEncoder().encode(data);
    let ascii: Uint8Array = encode(bytes);
    let rawString = decode(ascii);
    
    return data === rawString;
}
```

## Requirements

```bash
# make sure to have the wasm32 target installed
rustup target add wasm32-unknown-unknown

RUSTFLAGS=\"-C target-feature=+simd128 cargo test --target=wasm32-unknown-unknown
```

### Benchmarks
To run benchmarks, run `just bench`. It should lead you to a web page, you can view the console. 

The benchmark rules are very simple, it must follow `window.btoa` and `window.atob`'s function header: `String` -> `String`. 
Since certain functions have different function signatures, the work needed to convert into a `String` is included in the measurement. 

Codecs measured:
- `base64` with `wasm_bindgen` bindings
- `window.atob()`, `window.btoa()`
- `based64` `Uint8Array` -> `Uint8Array` with `wasm_bindgen` bindings
- `based64` `String` -> `String` with `wasm_bindgen` bindings

## Resources

- Mcyoung's [_Designing a SIMD Algorithm from Scratch_](https://mcyoung.xyz/2023/11/27/simd-base64/)
- Daniel Lemire's [_Ridiculously fast base64 encoding and decoding_](https://lemire.me/blog/2018/01/17/ridiculously-fast-base64-encoding-and-decoding/)
- [_core::arch::wasm32::v128_](https://doc.rust-lang.org/stable/core/arch/wasm32/struct.v128.html)
