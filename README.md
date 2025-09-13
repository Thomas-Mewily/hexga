🚧 **Warning: Experimental Crate!** 🚧

This crate is currently in **beta** and **experimental**.  
It is subject to **breaking changes** in future releases.  
Use it at your own risk, and keep in mind that the API may change in future versions.

## Toolings & Nightly

HexGa currently use nightly to have access to the `get_disjoint_mut_helpers` : [GetDisjointMutIndex trait from the slice::get_disjoint_mut()](https://doc.rust-lang.org/std/primitive.slice.html#method.get_disjoint_mut)

HexGa is **heavily trait-based**. For the best development experience, use the **latest version of Rust Analyzer** (the preview version on VS Code for example), which currently features the new trait solver. This makes autocompletion much faster.

## HexGa

HexGa is a set of crate to solve mutliple problem of different kind (mathematic,bitflags)...

To get started quickly, just do in a terminal :

```bash
cargo add hexga
```

then in your rust project :

```rust
use hexga::prelude::*;
```


HexGa (LibOur) stand for

- **H**ighly **E**xtensible **G**eneral **A**pplication **L**ib **O**ur

or

- **H**ighly **E**xtensible **GA**me **L**ib **O**ur


(Depending of the context)
The name was inspired by Excalibur, the sword.

Currently, HexGa is the continuation/start from scratch from a [school project](https://gitlab.isima.fr/thtamagnau/zz3_interpreter), as well as my personnal set of crate to do common task that I started way before the school project.