🚧 **Warning: Experimental Crate!** 🚧

This crate is currently in **beta** and **experimental**.
It is subject to **breaking changes** in future releases.
Use it at your own risk, and keep in mind that the API may change in future versions.

## HexGa Io

Io file abstraction based on [Hexga Encoding](https://crates.io/crates/hexga_encoding) `Load` and `Save` to allow loading/saving a value to a file.

It support custom user define extension and convertion, and it's also support common markup extension (json, ron, xml...).

Goal :
- Simple to use


```rust
use hexga_io::prelude::*;

"Hello file !".save_to_disk("./myfile.txt").unwrap();

let read = String::load_from_disk("./myfile.txt").unwrap();
assert_eq!("Hello file !", read);
```


```rust
// #[io] derive (serde::Serialize, serde::Deserialize) and (hexga_encoding::Load, hexga_encoding::Save)
#[io]
#[derive(PartialEq, Debug)]
struct Person
{
    age : i32,
    name : String,
}


let person = Person { age: 42, name: "Foo".to_owned() };
person.save_to_disk("./person.json").unwrap();
person.save_to_disk("./person.ron" ).unwrap();

assert_eq!(Person::load_from_disk("./person.ron"), Ok(person));
```

## Main Hexga crate

Check `hexga` : https://crates.io/crates/hexga if you are interested in a quick start, it regroup multiple hexga crates.