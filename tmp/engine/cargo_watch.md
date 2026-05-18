# Cargo watch on file change => Compile

```shell
cargo install cargo-watch
cargo install wasm-bindgen-cli

cd tmp/engine

# mode debug / le + rapide
cargo watch -s "wasm-pack build --target web --dev"

# peut être plus lent à compiler
cargo watch -s 'wasm-pack build --target web'

cargo watch -s "wasm-pack build --target web --dev --profile wasm-dev"

# + lancer web serveur sur vscode
```

# Different web server

```
cargo install trunk
```