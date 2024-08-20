TARGET := "wasm32-unknown-unknown"
SIMD128 := "RUSTFLAGS=\"-C target-feature=+simd128\""

test:
    {{ SIMD128 }} cargo test --target={{TARGET}}

expand:
    {{ SIMD128 }} cargo expand --target={{TARGET}}

build:
    {{ SIMD128 }} cargo build --target={{TARGET}}

build-wasm:
    {{ SIMD128 }} wasm-pack build --target web

clean-wasm:
    rm -rf pkg

bench:
    just clean-wasm && just build-wasm && rm -rf bench/pkg && cp -r pkg bench/ && go run bench/server.go

run:
    go run bench/server.go
