🚧 **Warning: Experimental Crate!** 🚧

This crate is currently in **beta** and **experimental**.  
It is subject to **breaking changes** in future releases.  
Use it at your own risk, and keep in mind that the API may change in future versions.


## HexGa Array

Define useful extensions methods on array, like :
-  `map_with`, (because there is no `zip` fn on array <https://github.com/rust-lang/rust/pull/112096> because zip is "eager" : doing `array1.zip(array2).map()` is bad, is is better to have only one function `map_with`, that will do both at once to avoid changing the array layout in memory for an array of tuple. Doing the operation on 2 different array allow the compiler to use SIMD instruction)

- `all_with`, `any_with`
- `get`, `get_mut`, `set`, `with`, `replace`...

```rust
use hexga_array::*;

assert_eq!([1,2,3].map_with([3,2,1],|a,b| a + b), [4,4,4]);
```

## Main Hexga crate

Check `hexga` : https://crates.io/crates/hexga if you are interested in a quick start, it regroup multiple hexga crates.