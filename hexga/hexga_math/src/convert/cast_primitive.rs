use super::*;

/// Macro to implement a "cast-to" trait for primitive numeric types
/// and for generic containers that implement `MapGeneric`.
macro_rules! impl_cast_to {
    ($trait_name:ident, $fn_map_name:ident, $fn_map_range_name:ident, $output_type:ty) => {
        /// Helper trait based on [CastInto] and [CastRangeInto]
        /// Also work on composite like [std::array], [Vector]...
        pub trait $trait_name : CastInto<Self::Output>
        {
            type Output;

            /// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html)
            /// keyword: `4f32 as u64`, and the [From] trait, but generic friendly.
            ///
            /// Like the [as](https://practice.course.rs/type-conversions/as.html) keyword, the result might lose some precision.
            fn $fn_map_name(self) -> Self::Output;
            /// Remap the value [RangeDefault] to the [RangeDefault] of the target type,
            /// in a generic friendly way, and similar to the [From] trait.
            fn $fn_map_range_name(self) -> Self::Output;
        }

        map_on_number_and_bool! {
            ($type_name:tt) => {
            impl $trait_name for $type_name {
                type Output = $output_type;
                fn $fn_map_name(self) -> Self::Output {
                    self.cast_into()
                }
                fn $fn_map_range_name(self) -> Self::Output {
                    self.cast_range_into()
                }
            }
        }}

        impl<S> $trait_name for S
        where
            S: $crate::map::MapGeneric + CastInto<S::WithType<<S::Item as $trait_name>::Output>>,
            S::Item: $trait_name,
        {
            type Output = S::WithType<<S::Item as $trait_name>::Output>;
            fn $fn_map_name(self) -> Self::Output {
                self.map($trait_name::$fn_map_name)
            }
            fn $fn_map_range_name(self) -> Self::Output {
                self.map($trait_name::$fn_map_range_name)
            }
        }
    };
}

impl_cast_to!(ToFloat, to_float, to_float_range, float);
impl_cast_to!(ToInt, to_int, to_int_range, int);
impl_cast_to!(ToUInt, to_uint, to_uint_range, uint);
impl_cast_to!(ToF32, to_f32, to_f32_range, f32);
impl_cast_to!(ToF64, to_f64, to_f64_range, f64);
impl_cast_to!(ToU8, to_u8, to_u8_range, u8);
impl_cast_to!(ToU16, to_u16, to_u16_range, u16);
impl_cast_to!(ToU32, to_u32, to_u32_range, u32);
impl_cast_to!(ToU64, to_u64, to_u64_range, u64);
impl_cast_to!(ToUSize, to_usize, to_usize_range, usize);
impl_cast_to!(ToI8, to_i8, to_i8_range, i8);
impl_cast_to!(ToI16, to_i16, to_i16_range, i16);
impl_cast_to!(ToI32, to_i32, to_i32_range, i32);
impl_cast_to!(ToI64, to_i64, to_i64_range, i64);
impl_cast_to!(ToISize, to_isize, to_isize_range, isize);
impl_cast_to!(ToBool, to_bool, to_bool_range, bool);