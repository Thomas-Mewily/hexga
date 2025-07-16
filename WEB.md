# WEB GL

```shell
cargo build --package test_graphics --target wasm32-unknown-unknown --release --features webgl
;
wasm-bindgen --target web --out-dir ./out ./target/wasm32-unknown-unknown/release/test_graphics.wasm

```


# WGPU

```shell
cargo build --package test_graphics --target wasm32-unknown-unknown --release
;
wasm-bindgen --target web --out-dir ./out ./target/wasm32-unknown-unknown/release/test_graphics.wasm

```
