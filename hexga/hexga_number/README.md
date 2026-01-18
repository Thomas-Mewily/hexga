

## HexGa Number

Provide basic number functionality, like

- Some constant in trait : `Zero` `One` `Half`, `MaxValue`, `MinValue`, `NaNValue`,

- Some trait that regroup multiple operation :

    - Type of operations
        - `BitArithmetic` : For every type that support bit based operation (and `&`, or `|`, xor `^`, not `!`, shift `<<` / `>>`...).
        - `UnitArithmetic` : +, -, 0
        - `NumberArithmetic` : +, -, *, /, %, 0 - `ArithmeticNegative` : same as NumberArithmetic + Neg operator,
        - `Number` : +, -, *, /, %, 0, 1, ==, >=, min val, max val,
        - `NumberNegative` same as Number + Neg operator,

    - Float and Int related : `Floating`, `Integer`, `IntegerUnsigned`, `IntegerSigned`

- Useful macro : `map_on_integer`, `map_on_integer_unsigned`, `map_on_integer_signed`, `map_on_float`, `map_on_number`


## Example using a map_on! macro

```rust
/// Define the `0` representation : The absorbing element of the multiplication such that `x * X::ZERO = X::ZERO`
pub trait Zero : Sized
{
    /// The absorbing element of the multiplication such that `x * X::ZERO = X::ZERO`
    const ZERO : Self;
}

map_on_number!(
    ($name:ident) =>
    {
        impl Zero for $name
        {
            const ZERO : Self = 0 as Self;
        }
    }
);
// and tada! Zero is now implemented for : (`i8`, `i16`, `i32`, `i64`, `isize`) + (`u8`, `u16`, `u32`, `u64`, `usize`) + (`f32`, `f64`)
```

## Main Hexga crate

Check `hexga` : https://crates.io/crates/hexga if you are interested in a quick start, it regroup multiple hexga crates.