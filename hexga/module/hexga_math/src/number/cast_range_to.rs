use crate::*;

/// Might lose some precision.
/// Cast the default range of the value to the target range of the other value
pub trait CastRangeIntoComposite<T>
{
    type Output;
    /// Might lose some precision.
    fn cast_range_into_composite(self) -> Self::Output;
}

pub trait CastRangeInto<T> : CastRangeIntoComposite<T,Output = T> + Sized { fn cast_range_into(self) -> Self::Output { self.cast_range_into_composite() } }
impl<T,T2> CastRangeInto<T> for T2 where T2 : CastRangeIntoComposite<T,Output = T> {}

pub trait CastRangeFrom<T> { fn cast_range_from(value : T) -> Self; }
impl<Src,Dest> CastRangeFrom<Dest> for Src where Dest : CastRangeInto<Src> { fn cast_range_from(value : Dest) -> Self { value.cast_range_into_composite() } }

impl_composite_output_with_methods!(CastRangeIntoComposite<CastToRangeOut>, cast_range_into_composite);



// Double recursive macro :)
macro_rules! impl_cast_range_to_integer 
{ 
    ($itself: ty, $cast_range_into: ty) => 
    { 
        impl CastRangeIntoComposite<$cast_range_into> for $itself
        {
            type Output = $cast_range_into;
            fn cast_range_into_composite(self) -> Self::Output 
            {
                if <$itself>::PRIMITIVE_NUMBER_TYPE.is_float() && <$cast_range_into>::PRIMITIVE_NUMBER_TYPE.is_float()
                {
                    // Same range
                    return $itself as $cast_range_into;
                }

                if <$itself>::PRIMITIVE_NUMBER_TYPE.is_float()
                {
                    
                }

                match (<$itself>::PRIMITIVE_NUMBER_TYPE, <$cast_range_into>::PRIMITIVE_NUMBER_TYPE)
                {

                }
                if std::mem::size_of::<$itself>() == std::mem::size_of::<$cast_range_into>() 
                { 
                    if std::mem::type_id::<$itself>() == std::mem::type_id::<$cast_range_into>()
                    {
                        // Noop, self is the same type as $cast_range_into
                        return self as $cast_range_into;
                    }

                    match 
                    if <$itself>::PRIMITIVE_NUMBER_TYPE.is_unsigned()
                    {

                    }

                }
                if <$itself>::PRIMITIVE_NUMBER_TYPE.is_float() || <$cast_range_into>::PRIMITIVE_NUMBER_TYPE.is_float()
                {
                    return <$cast_range_into>::from_coef(r.to_coef())
                }

                if std::mem::size_of::<$itself>() > std::mem::size_of::<$cast_range_into>() 
                {
                    // down cast
                    (self * (<$itself>::RANGE / (<$cast_range_into>::RANGE as $itself))) as $cast_range_into
                }else
                {
                    // up cast
                    ((self as $cast_range_into) * (<$cast_range_into>::RANGE / (<$itself>::RANGE as $cast_range_into)))
                }
            }
        }
    }; 

    ($cast_range_into: ty) => 
    {
        map_on_integer!(map_on_integer,$cast_range_into);
    }; 
}
map_on_integer!(impl_cast_range_to_integer);
