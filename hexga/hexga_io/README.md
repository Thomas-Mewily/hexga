🚧 **Warning: Experimental Crate!** 🚧

This crate is currently in **beta** and **experimental**.
It is subject to **breaking changes** in future releases.
Use it at your own risk, and keep in mind that the API may change in future versions.

## HexGa IO

Io file abstraction based on serde to allow loading/saving and converting bytes.

Is support custom user define extension and convertion, and it also support common markup extension (json, ron, xml...).

Goal :
- Simple to use
- Allow to save composite file

Non Goal :
- Async Loading (use some kind handle that will load it later instead, loading composite file async in a non blocking way is hard...)

```rust
use hexga_io::prelude::*;

"Hello file !".save_to_disk("./myfile.txt").unwrap();

let read = String::load_from_disk("./myfile.txt").unwrap();
assert_eq!("Hello file !", read);
```



```rust

// #[io] derive (serde::Serdialize, serde::Deserialize) and (hexga_io::Load, hexga_io::Save)
#[io]
#[derive(PartialEq, Debug)]
struct Person
{
    age : i32,
    name : String,
}

let mut fs = IoFsDisk::new();

    let person = Person { age: 42, name: "Life".to_owned() };
    person.save_to("./person.json", &mut fs).unwrap();
    person.save_to("./person.ron" , &mut fs).unwrap();

fs.commit().unwrap();

assert_eq!(Person::load_from_disk("./person.ron"), Ok(person));
```

## Main Hexga crate

Check `hexga` : https://crates.io/crates/hexga if you are interested in a quick start, it regroup multiple hexga crates.