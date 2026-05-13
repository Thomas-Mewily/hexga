#![no_std]

mod map_on;
mod map_on_common;
mod map_on_tuple;

pub mod prelude
{
    pub use crate::map_on;
    pub use crate::map_on_tuple;

    pub use crate::
    {
        map_on_integer_unsigned,
        map_on_integer_signed,
        map_on_integer,
        map_on_float,
        map_on_number,
        map_on_number_and_bool,
        map_on_number_and_bool_and_char,
        map_on_operator_binary_arithmetic_unit,
        map_on_operator_binary_arithmetic,
        map_on_operator_binary_bit,
        map_on_operator_binary,
        map_on_operator_assign_arithmetic_unit,
        map_on_operator_assign_arithmetic,
        map_on_operator_assign_bit,
        map_on_operator_assign,
        map_on_std_fmt_without_debug,
        map_on_std_fmt,
        map_on_operator_unary_bit,
        map_on_operator_unary_arithmetic_unit,
        map_on_operator_unary,
    };
}
