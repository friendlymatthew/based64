TARGET := "wasm32-unknown-unknown"

build:
    cargo build --target={{TARGET}}

test:
    cargo test --target={{TARGET}}

expand:
    cargo expand --target={{TARGET}}
