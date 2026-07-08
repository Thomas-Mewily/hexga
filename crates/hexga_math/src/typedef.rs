use super::*;

pub use hexga_typedef::*;

/// Macro to implement a "cast-to" trait for primitive numeric types
/// and for generic containers that implement `MapGeneric`.
macro_rules! impl_cast_into {
    (
        $trait_name:ident, $fn_map_name:ident, $fn_map_range_name:ident, $output_type:ty,
        $target_trait_name:ident, $target_fn_map_name:ident, $target_fn_map_range_name:ident, $target_output_type:ty
    ) => {
        /// Helper trait based on [`CastInto`] and [`CastRangeInto`]
        /// Also work on composite like [`std::array`], [`Vector`]...
        pub trait $trait_name: $target_trait_name
        {
            /// Same semantics as the [`as`](https://practice.course.rs/type-conversions/as.html)
            /// keyword: `4f32 as u64`, and the [`From`] trait, but generic friendly.
            ///
            /// Like the [`as`](https://practice.course.rs/type-conversions/as.html) keyword, the result might lose some precision.
            #[inline]
            fn $fn_map_name(self) -> Self::Output { self.$target_fn_map_name() }
            /// Remap the value [`RangeDefault`] to the [`RangeDefault`] of the target type,
            /// in a generic friendly way, and similar to the [`From`] trait.
            #[inline]
            fn $fn_map_range_name(self) -> Self::Output { self.$target_fn_map_range_name() }
        }
        impl<T> $trait_name for T where T: $target_trait_name {}
    };
}


#[cfg(feature = "int_are_8_bits")]
impl_cast_into!(ToInt, to_int, to_int_range, int, ToI8, to_i8, to_i8_range, i8);

#[cfg(feature = "int_are_16_bits")]
impl_cast_into!(ToInt, to_int, to_int_range, int, ToI16, to_i16, to_i16_range, i16);

#[cfg(feature = "int_are_32_bits")]
impl_cast_into!(ToInt, to_int, to_int_range, int, ToI32, to_i32, to_i32_range, i32);

#[cfg(feature = "int_are_64_bits")]
impl_cast_into!(ToInt, to_int, to_int_range, int, ToI64, to_i64, to_i64_range, i64);

#[cfg(feature = "int_are_size_bits")]
impl_cast_into!(ToInt, to_int, to_int_range, int, ToISize, to_isize, to_isize_range, isize);




#[cfg(feature = "int_are_8_bits")]
impl_cast_into!(ToUInt, to_uint, to_uint_range, uint, ToU8, to_u8, to_u8_range, u8);

#[cfg(feature = "int_are_16_bits")]
impl_cast_into!(ToUInt, to_uint, to_uint_range, uint, ToU16, to_u16, to_u16_range, u16);

#[cfg(feature = "int_are_32_bits")]
impl_cast_into!(ToUInt, to_uint, to_uint_range, uint, ToU32, to_u32, to_u32_range, u32);

#[cfg(feature = "int_are_64_bits")]
impl_cast_into!(ToUInt, to_uint, to_uint_range, uint, ToU64, to_u64, to_u64_range, u64);

#[cfg(feature = "int_are_size_bits")]
impl_cast_into!(ToUInt, to_uint, to_uint_range, uint, ToUSize, to_usize, to_usize_range, usize);



#[cfg(feature = "float_are_32_bits")]
impl_cast_into!(ToFloat, to_float, to_float_range, float, ToF32, to_f32, to_f32_range, f32);

#[cfg(feature = "float_are_64_bits")]
impl_cast_into!(ToFloat, to_float, to_float_range, float, ToF64, to_f64, to_f64_range, f64);

#[cfg(feature = "float_are_size_bits")]
impl_cast_into!(ToFloat, to_float, to_float_range, float, ToF64, to_f64, to_f64_range, f64);

/*
impl_cast_into!(ToFloat, to_float, to_float_range, float);
impl_cast_into!(ToUInt, to_uint, to_uint_range, uint);
*/