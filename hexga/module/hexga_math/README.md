🚧 **Warning: Experimental Crate!** 🚧

This crate is currently in **beta** and **experimental**.
It is subject to **breaking changes** in future releases.
Use it at your own risk, and keep in mind that the API may change in future versions.

# HexGa Math

Check [the documentation](https://docs.rs/hexga_math/latest/hexga_math/) to find some examples in the doc.

## A Math library that contains :

### N Dimension stuff

This crate define N dimensionnal math stuff (2d, 3d, 4d, ... nd) like vector/point of any type (float, int, uint, or even user defined) :
- [Vector](https://docs.rs/hexga_math/latest/hexga_math/vector/index.html) (fixed size array wrapper)
- [Rectangle](https://docs.rs/hexga_math/latest/hexga_math/rectangle/struct.RectangleBase.html)
- [Grid](https://docs.rs/hexga_math/latest/hexga_math/grid/index.html)
- [Matrix](https://docs.rs/hexga_math/latest/hexga_math/matrix/index.html)

### Useful type like
- [Angle](https://docs.rs/hexga_math/latest/hexga_math/angle/struct.AngleOf.html),
- [Time](https://docs.rs/hexga_math/latest/hexga_math/time/struct.TimeOf.html),
- [ColorRGBA] with any precision (also handle the conversion between different primitive precision)
- [ColorHSLA] of various precision

### Generic Casting trait
The crate also provide generic traits for casting with the same behavior as the [as keyword](https://practice.course.rs/type-conversions/as.html) :
- [CastInto], [CastFrom] and [CastIntoComposite],

### Generic Remapping trait
Similar traits for casting remapping the range of an primitive to another primitive range also exist :
- [CastRangeInto], [CastRangeFrom] and [CastRangeIntoComposite]

### Quick start with the prelude
There are some quick typedef in the prelude :
- [int], [uint] and [float]  : The default primitive precision used in the typedef. (can be change with the feature flags)
- [Point2], [Point3], [Point4] for Vector of [int],
- [Vec2], [Vec3], [Vec4] for Vector of [float],
- [Rect2], [Rect3], [Rect4] for Rectangle of [float],
- [Rect2P], [Rect3P], [Rect4P] for Rectangle of [int] (`P` for point),
- [Mat2], [Mat3], [Mat4] for Matrix of [float], and [Mat2P], [Mat3P], [Mat4P] use [int],
- [Grid2], [Grid3], [Grid3] can only be indexed by [Point] by default.


### More advanced type
If you need more control about the precision, each type have another more generic long base type :
- [Grid] type uses a [Point] for the indexing precision, but that can be changed by using with the [GridBase] type.
- [Angle] and [Time] use a [float] precision that can be changed using [AngleOf] and [TimeOf]
- [ColorRGBA] and [ColorHSLA] also use a [float] precision that can be changed using [ColorRGBAOf] and [ColorRGBAOf]

Provide math related structure for multiple dimension, with a lot of typedef to them like :

- Vector (`Vec2`,`Vec3`,`Vec4` for a vector of float,... `Point2`,`Point3`,`Point4` for int...)

- Rectangle (`Rect2`,`Rect3`,`Rect4` for float, `Rect2P`,`Rect3P`,`Rect4P` for int/point)

- Matrix (`Matrix`, `Mat2`,`Mat3`,`Mat4` for float, `Mat2P`,`Mat3P`,`Mat4P` for int , `SquareMatrix`...)

- Grid (`Grid2`, `Grid3`, `Grid4`...)

Also provide

- some lightweight unit of measurement : `Time`, `Angle`.
`assert_eq!([1,2].degree(), [1.degree(),2.degree()])`

- some prefix : kilo, giga, mega...
`assert_eq!(1.kilo(), 1000)`,


## Main Hexga crate

Check `hexga` : https://crates.io/crates/hexga if you are interested in a quick start, it regroup multiple hexga crates.