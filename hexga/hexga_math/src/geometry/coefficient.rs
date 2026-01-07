use super::*;

/// Default value is One.
///
/// Same for missing value during deserialization.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Coef<T>
{
    pub value: T
}
impl<T> Coef<T>
{
    #[inline(always)]
    pub fn new(value: T) -> Self { Self { value }}
}
impl<T> From<T> for Coef<T>
{
    fn from(value: T) -> Self { Self::new(value) }
}
impl<T> Default for Coef<T> where T: One
{
    fn default() -> Self { Self::new(T::ONE) }
}


map_on_number_and_bool! {
    ($type_name:tt) => {
        #[cfg(feature = "serde")]
        impl ::serde::Serialize for Coef<$type_name>
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                self.value.serialize(serializer)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> ::serde::Deserialize<'de> for Coef<$type_name>
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                Ok(Self::new($type_name::deserialize(deserializer)?))
            }
        }
    }
}

/*
impl<W, T, const N: usize> Array<T, N> for Coef<W> where W: Array<T, N>, Self: From<[T;N]>, [T;N]: From<Self>
{
    fn array(&self) -> &[T; N] {
        self.value.array()
    }

    fn array_mut(&mut self) -> &mut[T; N] {
        self.value.array_mut()
    }
}
impl<W, T, const N: usize> ArrayWithType<T, N> for Coef<W> where W: ArrayWithType<T, N>, Self: From<[T;N]>, [T;N]: From<Self>
{
    type WithType<T2> = Coef<W::WithType<T2>>;
}

impl<W,const N : usize, Idx> Index<Idx> for Coef<W> where W: Index<Idx>
{
    type Output=<W as Index<Idx>>::Output;
    #[inline(always)]
    fn index(&self, index: Idx) -> &Self::Output { self.array().index(index) }
}
impl<W,const N : usize, Idx> IndexMut<Idx> for Coef<W> where W: IndexMut<Idx>
{
    #[inline(always)]
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output { self.array_mut().index_mut(index) }
}
*/