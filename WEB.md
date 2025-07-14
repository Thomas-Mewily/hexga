# WEB GL

```shell
cargo build --package test_graphics --target wasm32-unknown-unknown --release
;
wasm-bindgen --target web --out-dir ./out ./target/wasm32-unknown-unknown/release/wgpu_template.wasm

```


# WGPU

```shell
cargo build --target wasm32-unknown-unknown --release
;
wasm-bindgen --target web --out-dir ./out ./target/wasm32-unknown-unknown/release/wgpu_template.wasm

```


