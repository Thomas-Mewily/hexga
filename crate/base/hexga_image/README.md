🚧 **Warning: Experimental Crate!** 🚧

This crate is currently in **beta** and **experimental**.
It is subject to **breaking changes** in future releases.
Use it at your own risk, and keep in mind that the API may change in future versions.


## HexGa Graphics

Currently don't do that much, can just create a image from a grid and save it.


```rust
use hexga_image::prelude::*;

assert_eq!(<ColorRGBAOf::<u8> as CastRangeIntoComposite<u16>>::cast_range_into_composite(ColorRGBAOf::<u8>::RED),
            ColorRGBAOf::<u16>::RED
          );
```

### More advanced type

If you need more control about the precision, each type have another more generic base type:

- `ColorRGBA` and `ColorHSLA` also use a `float` precision that can be changed using `ColorRGBAOf` and `ColorRGBAOf`

## Main Hexga crate

Check `hexga` : https://crates.io/crates/hexga if you are interested in a quick start, it regroup multiple hexga crates.