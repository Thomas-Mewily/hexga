# Web with Wgpu

```shell
cargo build --target wasm32-unknown-unknown --release --package=wgpu_test2
;
wasm-bindgen --package=wgpu_test2 --target web --out-dir ./out ./target/wasm32-unknown-unknown/release/wgpu_template.wasm

```




cargo build --target wasm32-unknown-unknown --release --features webgl

