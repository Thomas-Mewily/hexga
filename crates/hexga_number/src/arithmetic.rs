use super::*;

pub trait RemEuclid<Rhs = Self>
{
    type Output;
    fn rem_euclid(self, rhs: Rhs) -> Self::Output;
}

map_on_number!(($primitive_name: ty) => 
{ 
    impl RemEuclid for $primitive_name 
    {
        type Output = Self;
        fn rem_euclid(self, rhs: Self) -> Self::Output { self.rem_euclid(rhs) }
    } 
});

impl<T> RemEuclid for Wrapping<T>
where
    T: RemEuclid,
{
    type Output = Wrapping<T::Output>;
    fn rem_euclid(self, rhs: Self) -> Self::Output { Wrapping(self.0.rem_euclid(rhs.0)) }
}
impl<T> RemEuclid for Saturating<T>
where
    T: RemEuclid,
{
    type Output = Saturating<T::Output>;
    fn rem_euclid(self, rhs: Self) -> Self::Output { Saturating(self.0.rem_euclid(rhs.0)) }
}


pub trait Pow<Exp = Self>
{
    type Output;
    fn pow(self, exp: Exp) -> Self;
}