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
    fn pow(self, exp: Exp) -> Self::Output;
}

map_on_integer!(($primitive_name: ty) => 
{ 
    impl Pow for $primitive_name 
    {
        type Output = Self;
        fn pow(self, exp: Self) -> Self::Output { self.pow(exp as _) }
    } 
});

map_on_float!(($primitive_name: ty) => 
{ 
    impl Pow for $primitive_name 
    {
        type Output = Self;
        fn pow(self, exp: Self) -> Self::Output { self.powf(exp) }
    }
});

impl<T> Pow for Wrapping<T>
where
    T: Pow,
{
    type Output = Wrapping<T::Output>;
    fn pow(self, rhs: Self) -> Self::Output { Wrapping(self.0.pow(rhs.0)) }
}
impl<T> Pow for Saturating<T>
where
    T: Pow,
{
    type Output = Saturating<T::Output>;
    fn pow(self, rhs: Self) -> Self::Output { Saturating(self.0.pow(rhs.0)) }
}