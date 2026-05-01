🚧 **Warning: Experimental Crate!** 🚧

This crate is currently in **beta** and **experimental**.
It is subject to **breaking changes** in future releases.
Use it at your own risk, and keep in mind that the API may change in future versions.

## HexGa Encoding

**hexga_encoding** provides a unified way to load, save, and encode a value across multiple formats and media types.

It includes support for [Data URLs (RFC 2397)](https://datatracker.ietf.org/doc/html/rfc2397), binary URLs, MIME-based media typing, and optional Serde integration.

Also export common Serde serializers and deserializers in a unified way (json, ron, ...) in the `serde` flag is present.

### 🧩 Example

```rust
use hexga_encoding::{ToUrl, FromUrl, Load, Save};

let text = String::from("Hello, world!");
let url = text.to_url("txt").unwrap();
assert!(url.starts_with("data:text/txt;base64,"));

let decoded = String::from_url(&url).unwrap();
assert_eq!(decoded, "Hello, world!");
```


## Main Hexga crate

Check `hexga` : https://crates.io/crates/hexga if you are interested in a quick start, it regroup multiple hexga crates.