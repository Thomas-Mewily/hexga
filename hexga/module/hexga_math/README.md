🚧 **Warning: Experimental Crate!** 🚧

This crate is currently in **beta** and **experimental**.
It is subject to **breaking changes** in future releases.
Use it at your own risk, and keep in mind that the API may change in future versions.

# HexGa Math

Check [the documentation](https://docs.rs/hexga_math/latest/hexga_math/) to find some examples in the doc.

## A Math library that contains :

### N Dimension stuff

This crate define N dimensionnal math stuff (2d, 3d, 4d, ... nd) like vector/point of any type (float, int, uint, or even user defined):

- [Vector](https://docs.rs/hexga_math/latest/hexga_math/vector/index.html) (fixed size array wrapper)
- [Rectangle](https://docs.rs/hexga_math/latest/hexga_math/rectangle/struct.RectangleBase.html)
- [Grid](https://docs.rs/hexga_math/latest/hexga_math/grid/index.html)
- [Matrix](https://docs.rs/hexga_math/latest/hexga_math/matrix/index.html)

### Useful type like
- [Angle](https://docs.rs/hexga_math/latest/hexga_math/angle/struct.AngleOf.html),
- [Time](https://docs.rs/hexga_math/latest/hexga_math/time/struct.TimeOf.html),
- [ColorRGBA] with any precision (also handle the conversion between different primitive precision)
- [ColorHSLA] of various precision

```rust
use hexga_math::prelude::*;

assert_eq!([1,2].degree(), [1.degree(),2.degree()]);
assert_eq!(1.kilo(), 1000);
```

### Generic Casting trait
The crate also provide generic traits for casting with the same behavior as the [as keyword](https://practice.course.rs/type-conversions/as.html) :
- [CastInto](https://docs.rs/hexga_math/latest/hexga_math/number/trait.CastInto.html), [CastFrom](https://docs.rs/hexga_math/latest/hexga_math/number/trait.CastFrom.html) and [CastIntoComposite](https://docs.rs/hexga_math/latest/hexga_math/number/trait.CastIntoComposite.html),

```rust
use hexga_math::prelude::*;

assert_eq!(i32::cast_from(255u8), 255i32);
assert_eq!(i32::cast_from(12.3f32), 12);

let vec_f32 = Vector2::<f32>::new(0.5, 0.5);
let vec_f64 = Vector2::<f64>::new(0.5, 0.5);
let vec_f32_to_f64 = <Vector2::<f32> as CastIntoComposite<f64>>::cast_into_composite(vec_f32);
assert_eq!(vec_f32_to_f64, vec_f64);
```

### Generic Remapping trait
Similar traits for casting remapping the range of an primitive to another primitive range also exist :
- [CastRangeInto](https://docs.rs/hexga_math/latest/hexga_math/number/trait.CastRangeInto.html), [CastRangeFrom](https://docs.rs/hexga_math/latest/hexga_math/number/trait.CastRangeFrom.html) and [CastRangeIntoComposite](https://docs.rs/hexga_math/latest/hexga_math/number/trait.CastRangeIntoComposite.html)

```rust
use hexga_math::prelude::*;

assert_eq!(u8::cast_range_from(1f32), 255u8);
assert_eq!(u8::cast_range_from(127i8), 254u8);
assert_eq!(i8::cast_range_from(255u8), 127i8);

assert_eq!(<ColorRGBAOf::<u8> as CastRangeIntoComposite<u16>>::cast_range_into_composite(ColorRGBAOf::<u8>::RED),
            ColorRGBAOf::<u16>::RED
          );
```


### Quick start with the prelude

There are some quick typedef in the prelude :
- `int`, `uint` and `float`  : The default primitive precision used in the typedef. (can be change with the feature flags)
- `Point2`, `Point3`, `Point4` for Vector of `int`,
- `Vec2`, `Vec3`, `Vec4` for Vector of `float`,
- `Rect2`, `Rect3`, `Rect4` for Rectangle of `float`,
- `Rect2P`, `Rect3P`, `Rect4P` for Rectangle of `int` (`P` for point),
- `Mat2`, `Mat3`, `Mat4` for Matrix of `float`, and `Mat2P`, `Mat3P`, `Mat4P` use `int`,
- `Grid2`, `Grid3`, `Grid3` can only be indexed by `Point` by default.


### More advanced type

If you need more control about the precision, each type have another more generic long base type:

- `Grid` type uses a `Point` for the indexing precision, but that can be changed by using with the `GridBase` type.
- `Angle` and `Time` use a `float` precision that can be changed using `AngleOf` and `TimeOf`
- `ColorRGBA` and `ColorHSLA` also use a `float` precision that can be changed using `ColorRGBAOf` and `ColorRGBAOf`

## Main Hexga crate

Check `hexga` : https://crates.io/crates/hexga if you are interested in a quick start, it regroup multiple hexga crates.