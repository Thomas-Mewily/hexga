//! ## HexGa Map On
//!
//! The core `map_on!` macro allows you to iterate over a list of tokens and generate repetitive code patterns.
//! This is particularly useful for avoiding repetitive boilerplate code when implementing traits for multiple primitive types for example.
//!
//! The fundamental idea is simple: take a list of tokens and apply a macro to each one.
//! This is useful for avoiding boilerplate when implementing traits for multiple types:
//!
//! ```rust
//! use hexga_map_on::*;
//!
//! trait MinusOne
//! {
//!     const MINUS_ONE : Self;
//! }
//!
//! map_on!
//! (
//!     (
//!         i8, i16, i32, i64, isize,
//!         f32, f64
//!     ),
//!     ($name:ident) =>
//!     {
//!         impl MinusOne for $name
//!         {
//!             const MINUS_ONE : Self = -1 as Self;
//!         }
//!     }
//! );
//! ```
//!
//! Some variation of `map_on` exist for different use case :
//!
//! ```rust
//! use hexga_map_on::*;
//!
//! trait Zero
//! {
//!     const ZERO : Self;
//! }
//!
//! map_on_number!(
//!     ($name:ident) =>
//!     {
//!         impl Zero for $name
//!         {
//!             const ZERO : Self = 0 as Self;
//!         }
//!     }
//! );
//! ```
//!
//! ## More Example
//!
//! You can also use the `map_on!` macro to call another macro :
//!
//! ```rust
//! use hexga_map_on::*;
//!
//! trait One
//! {
//!     const ONE : Self;
//! }
//!
//! macro_rules! impl_one {
//!     ($type_name:ty) => {
//!         impl One for $type_name
//!         {
//!             const ONE : Self = 1 as Self;
//!         }
//!     };
//! }
//!
//! map_on_number!(impl_one);
//! ```
//!
//! It is also possible to use nested `map_on!`*1 macro (only when using macro name, or in const context like implementing a trait) :
//!
//! ```rust
//! use hexga_map_on::*;
//!
//! pub trait CastInto<T>
//! {
//!     /// Might lose some precision.
//!     /// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html) keyword: `4f32 as u64`
//!     fn cast_into(self) -> T;
//! }
//!
//! // Double recursive macro :)
//! macro_rules! impl_cast_into
//! {
//!     ($itself: ty, $cast_into: ty) =>
//!     {
//!         impl CastInto<$cast_into> for $itself
//!         {
//!             fn cast_into(self) -> $cast_into { self as _ }
//!         }
//!     };
//!
//!     ($cast_into: ty) =>
//!     {
//!         map_on_number!(impl_cast_into,$cast_into);
//!     };
//! }
//! // Do 144 trait impl in a few lines :)
//! map_on_number!(impl_cast_into);
//!
//! fn main()
//! {
//!     assert_eq!(20.5f32 as i8, 20.5f32.cast_into());
//!     assert_eq!(4.5 as u32, 4.5.cast_into());
//!     assert_eq!(4u8 as i64, 4u8.cast_into());
//! }
//! ```
//!
//! Implementing a binary operator is also possible :
//!
//! ```rust
//! use hexga_map_on::*;
//!
//! #[derive(Debug)]
//! struct X(pub i32);
//!
//! map_on_operator_binary!(
//!     (($trait_name: tt, $fn_name: tt)) =>
//!     {
//!         impl std::ops::$trait_name for X
//!         {
//!             type Output = X;
//!             fn $fn_name(self, rhs : Self) -> Self::Output { X(self.0.$fn_name(rhs.0)) }
//!         }
//!     }
//! );
//!
//! fn main()
//! {
//!     let x =         X(9) + X(3) * X(4) / X(2);
//!     assert_eq!(x.0,   9  +   3  *   4  /   2 );
//! }
//! ```
//!
//! Check the `example` folder for more examples.
//!
//! ## Limitation
//!
//! Right now it is impossible to use the `map_on!` macro in a non const context (like in a function body) with lambda syntax.
//!
//! ```rust
//! macro_rules! print_type {
//!     ($type_name:ty) => {
//!         println!("print type from macro name {}", ::std::any::type_name::<$type_name>());
//!     };
//! }
//!
//! fn main()
//! {
//!     // work fine
//!     map_on!((f32, f64), print_type);
//!
//!     // Don't work :/
//!     map_on!((f32, f64),
//!         ($type_name:ident) =>
//!         {
//!             println!("print type from macro lambda {}", ::std::any::type_name::<$type_name>());
//!         }
//!     );
//! }
//! ```
//!
//! The reason is that the lambda macro form will create a temporary macro with the name `__map_on_inliner`.
//! Because nested lambda map_on! macro call will generate a new macro each time with the same name `__map_on_inliner`, it will conflict with the previous one.
//! So we need a mecansime to scope the macro name.
//!
//! Right now the only way to do that that I know is to use a `const` block, which is not ideal because it limit where the macro can be used : in a const context.
//!
//! ```rust
//! const _: () = {
//!     // definitions, trait impls, etc…
//! };
//! ```
//! <https://internals.rust-lang.org/t/anonymous-modules/15441/2?u=thomas-mewily>
//!
//! The definiton for lambda `map_on!` macro is the following :
//!
//! ```rust
//! ($tokens:tt, $($macro_arms:tt)+) => {
//!     const _: () = {
//!         macro_rules! __map_on_inliner {
//!             $($macro_arms)+
//!         }
//!
//!         $crate::map_on!(@expand_tokens $tokens);
//!     };
//! };
//! ```
//!
//! This is the kind of stuff I was doing in C, ported to Rust while also using the full power of Rust macros.
//!
//! The official name for this technique seem to be `X macro`, but since the main focus is mapping tokens over tokens I prefer the name `map_on macro`.
//! https://en.wikipedia.org/wiki/X_macro
//!

#![no_std]

mod map_on;
mod map_on_common;
mod map_on_tuple;

pub mod prelude
{
    pub use crate::map_on;
    pub use crate::map_on_tuple;

    pub use crate::{
        map_on_float, map_on_integer, map_on_integer_signed, map_on_integer_unsigned, map_on_number, map_on_number_and_bool, map_on_number_and_bool_and_char,
        map_on_operator_assign, map_on_operator_assign_arithmetic, map_on_operator_assign_arithmetic_unit, map_on_operator_assign_bit, map_on_operator_binary,
        map_on_operator_binary_arithmetic, map_on_operator_binary_arithmetic_unit, map_on_operator_binary_bit, map_on_operator_unary,
        map_on_operator_unary_arithmetic_unit, map_on_operator_unary_bit, map_on_std_fmt, map_on_std_fmt_without_debug,
    };
}
