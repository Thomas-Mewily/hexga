
on the web `rustup target add wasm32-unknown-unknown`

# Web with Wgpu

```shell
cargo build --target wasm32-unknown-unknown --release --package=wgpu_test2
&&
wasm-bindgen --target web --out-dir ./out ./target/wasm32-unknown-unknown/release/wgpu_test2.wasm
```

## To automatically recompile the wasm when a file change

```shell
cargo install cargo-watch --locked


cargo watch  -i out -x "build --target wasm32-unknown-unknown --release --package wgpu_test2" -s "wasm-bindgen --target web --out-dir ./out ./target/wasm32-unknown-unknown/release/wgpu_test2.wasm"


# -i : ignore le dossier out
# -x : commande cargo
# -s : commande shell
```


