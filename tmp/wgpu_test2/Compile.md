
# Web

## Setup (once)

on the web `rustup target add wasm32-unknown-unknown`

## Web with Wgpu

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

# Android

## Setup (once)

1) Install the rust android toolchain
```shell
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
```

2) Install Android NDK + SDK

Install android studio, then in SDK Manager install NDK (Side by side) and CMake.

Set some env var
```
export ANDROID_HOME=$HOME/Android/Sdk
export NDK_HOME=$ANDROID_HOME/ndk/<version>

(home is somethings like C:\Users\USERNAME\AppData\Local\)
```

3) cargo-ndk

```
cargo ndk -t arm64-v8a -o ./app/src/main/jniLibs build --release

```
