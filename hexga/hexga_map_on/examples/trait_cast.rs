use hexga_map_on::prelude::*;

// Similar to Trait From / Into, but those trait suppose no loss when converting, so it is impossible to cast a f32 to a i64 for exemple

/// Might lose some precision.
/// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html) keyword: `4f32 as u64`
pub trait CastInto<T>
{
    /// Might lose some precision.
    /// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html) keyword: `4f32 as u64`
    fn cast_to(self) -> T;
}

// Double recursive macro :)
macro_rules! impl_cast_to 
{ 
    ($itself: ty, $cast_into: ty) => 
    { 
        impl CastInto<$cast_into> for $itself
        {
            fn cast_to(self) -> $cast_into { self as _ }
        }
    }; 

    ($cast_into: ty) => 
    {
        map_on_number!(impl_cast_to,$cast_into);
    }; 
}
// Do 144 trait impl in a few lines :) 
map_on_number!(impl_cast_to);

fn main()
{
    assert_eq!(20.5f32 as i8, 20.5f32.cast_to());

    assert_eq!(4.5 as u32, 4.5.cast_to());
    assert_eq!(4u8 as i64, 4u8.cast_to());
}

/* 
// If you also want to support bool
macro_rules! impl_cast_to_bool 
{ 
    ($itself: ty) => 
    {
        impl CastInto<bool> for $itself
        {
            fn cast_to(self) -> bool { self == (0 as $itself) }
        }
    };
}
map_on_number!(impl_cast_to_bool);
impl CastInto<bool> for bool { fn cast_to(self) -> bool { self } }
*/