# based64

A base64 codec using wasm32 SIMD intrinsics.

```rust
use based64;
use anyhow;

fn main() -> anyhow::Result<()> {
  let ascii = b"VGhlIGRvZyBsaWNrZWQgdGhlIG9pbCwgYW5kIGV2ZXJ5Ym9keSBsYXVnaGVkLg==";
  let message = decode(ascii)?; // The dog licked the oil, and everybody laughed.

  let encoded_to_ascii = encode(&message)?;
  assert_eq!(encoded_to_ascii, ascii.to_vec());

  Ok(())
}
```

## Requirements

```bash
# make sure to have the wasm32 target installed
rustup target add wasm32-unknown-unknown

RUSTFLAGS=\"-C target-feature=+simd128 cargo test --target=wasm32-unknown-unknown
```

## Resources

- Mcyoung's [_Designing a SIMD Algorithm from Scratch_](https://mcyoung.xyz/2023/11/27/simd-base64/)
- Daniel Lemire's [_Ridiculously fast base64 encoding and decoding_](https://lemire.me/blog/2018/01/17/ridiculously-fast-base64-encoding-and-decoding/)
- [_core::arch::wasm32::v128_](https://doc.rust-lang.org/stable/core/arch/wasm32/struct.v128.html)
