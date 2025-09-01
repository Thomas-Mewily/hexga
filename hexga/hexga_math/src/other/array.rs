use super::*;

pub use hexga_array::prelude::*;


pub mod prelude { pub use super::*; }

/* 
/// A vector, where each dimension can have a name (Interpretation)
/// The `Interpretation` is just a plain old data, without any validation
#[repr(C)]
pub struct NamedVector<T, Interpretation, const N : usize>
{
    // Not simd because we accept everythings.
    pub array : [T; N],
    pub(crate) no_destructuring : PhantomData<Interpretation>,
}

impl<T, I, const N : usize> Default for NamedVector<T,I,N> where I:Default
{
    fn default() -> Self { Self::from_value(___()) }
}

impl<T, I, const N : usize> Array<T,N> for NamedVector<T,I,N>
{
    fn array(&self) -> &[T; N] { &self.array }

    fn array_mut(&mut self) -> &mut[T; N] { &mut self.array }
}

impl<T, I, const N : usize, Idx> Index<Idx> for NamedVector<T,I,N> where [T;N]: Index<Idx>
{
    type Output = <[T;N] as Index<Idx>>::Output;
    fn index(&self, index: Idx) -> &Self::Output { &self.array[index] }
}
impl<T, I, const N : usize, Idx> IndexMut<Idx> for NamedVector<T,I,N>  where [T;N]: IndexMut<Idx>
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output { &mut self.array[index] }
}
impl<T, I, const N : usize, Idx> Get<Idx> for NamedVector<T,I,N> where [T;N]: Get<Idx>
{
    type Output = <[T;N] as Get<Idx>>::Output;

    unsafe fn get_unchecked(&self, index : Idx) -> &Self::Output { unsafe { self.array.get_unchecked(index) } }
    fn get(&self, index : Idx) -> Option<&Self::Output> { self.array.get(index) }
    #[inline(always)]
    #[track_caller]
    fn try_get(&self, index : Idx) -> Result<&Self::Output, ()> { self.array.try_get(index) }
}
impl<T, I, const N : usize, Idx> GetMut<Idx> for NamedVector<T,I,N> where [T;N]: GetMut<Idx>
{
    fn try_get_mut(&mut self, index : Idx) -> Result<&mut Self::Output, ()> {
        self.array.try_get_mut(index)
    }
}
impl<T, I, const N : usize, Idx> GetManyMut<Idx> for NamedVector<T,I,N> where [T;N]: GetManyMut<Idx>
{
    fn try_get_many_mut<const N2: usize>(&mut self, indices: [Idx; N2]) -> Result<[&mut Self::Output;N2], ()> {
        self.array.try_get_many_mut(indices)
    }
}

impl<T, I, const N : usize> IntoIterator for NamedVector<T, I, N>
{
    type Item = <[T;N] as IntoIterator>::Item;
    type IntoIter = <[T;N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { self.array.into_iter() }
}

impl<'a, T, I, const N: usize> IntoIterator for &'a NamedVector<T, I, N> 
{
    type Item = <&'a [T; N] as IntoIterator>::Item;
    type IntoIter = <&'a [T; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { (&self.array).into_iter() }
}

impl<'a, T, I, const N: usize> IntoIterator for &'a mut NamedVector<T, I, N> 
{
    type Item = <&'a mut [T; N] as IntoIterator>::Item;
    type IntoIter = <&'a mut [T; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { (&mut self.array).into_iter() }
}

impl<T, I, const N : usize> FromIterator<T> for NamedVector<T, I, N> where T:Default
{
    fn from_iter<It: IntoIterator<Item = T>>(iter: It) -> Self 
    {
        let mut it = iter.into_iter();
        Self::from_array(std::array::from_fn(|_| it.next().unwrap_or_default()))
    }
}



impl<T, I, const N : usize>  NamedVector<T, I, N>
{
    pub(crate) valid : () = const fn assert_valid_interpretation();
    #[track_caller]
    #[inline]
    pub(crate) const fn assert_valid_interpretation()
    {
        assert!(std::mem::size_of::<[T; N]>() == std::mem::size_of::<I>(),"NamedVector: [T; N] and Interpretation must have the same size");
        assert!(std::mem::size_of::<[T; N]>() == std::mem::size_of::<I>(),"NamedVector: [T; N] and Interpretation must have the same alignement");
    }

    const fn from_array(array : [T;N]) -> Self 
    {
        Self::assert_valid_interpretation();
        Self { array, no_destructuring: PhantomData }
    }

    const fn from_value(value: I) -> Self 
    {
        Self::assert_valid_interpretation();
        let array = unsafe {
            let ptr = &value as *const I as *const [T; N];
            std::ptr::read(ptr)
        };
        std::mem::forget(value);
        Self { array, no_destructuring: PhantomData }
    }

    const fn into_value(self) -> I
    {
        Self::assert_valid_interpretation();
        let value = unsafe {
            let ptr = &self.array as *const [T; N] as *const I;
            std::ptr::read(ptr)
        };
        std::mem::forget(self);
        value
    }
}

impl<T, I, const N : usize> Deref for NamedVector<T,I,N>
{
    type Target=I;
    fn deref(&self) -> &Self::Target 
    {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T, I, const N : usize> DerefMut for NamedVector<T,I,N>
{
    fn deref_mut(&mut self) -> &mut Self::Target 
    {
        unsafe { std::mem::transmute(self) }
    }
}


// Can't impl `From<I> for NamedVector` and `From<NamedVector> for I` because of impl conflict if I is already an array, or Self
impl<T, I, const N : usize> From<[T;N]> for NamedVector<T,I,N>
{
    fn from(value: [T;N]) -> Self { Self::from_array(value) }
}

impl<T, I, const N : usize> From<NamedVector<T,I,N>> for [T;N]
{
    fn from(value: NamedVector<T,I,N>) -> Self  { value.array }
}

impl<T, I, const N : usize> AsRef<[T;N]> for NamedVector<T,I,N>
{
    fn as_ref(&self) -> &[T;N] { &self.array }
}
impl<T, I, const N : usize> AsMut<[T;N]> for NamedVector<T,I,N>
{
    fn as_mut(&mut self) -> &mut [T;N] { &mut self.array }
}

map_on_std_fmt!(
    ($trait_name:ident) => 
    {
        impl<T, I, const N : usize> std::fmt::$trait_name for NamedVector<T,I,N> where I: std::fmt::$trait_name
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
            {
                self.deref().fmt(f)
            }
        }
    } 
);



impl<T, I, const N : usize> Copy for NamedVector<T, I, N> where [T; N] : Copy {}
impl<T, I, const N : usize> Clone for NamedVector<T, I, N> where [T; N] : Clone 
{
    fn clone(&self) -> Self 
    {
        Self { array: self.array.clone(), no_destructuring: PhantomData }
    }
}

impl<T, I, const N : usize> PartialEq for NamedVector<T, I, N> where [T; N] : PartialEq
{
    fn eq(&self, other: &Self) -> bool { self.array == other.array }
}
impl<T, I, const N : usize> Eq for NamedVector<T, I, N> where [T; N] : Eq {}

impl<T, I, const N : usize> Hash for NamedVector<T, I, N> where [T; N] : Hash 
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.array.hash(state);
    }
}

impl<T, I, const N : usize> PartialOrd for NamedVector<T, I, N> where [T; N] : PartialOrd 
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.array.partial_cmp(&other.array)
    }
}
impl<T, I, const N : usize> Ord for NamedVector<T, I, N> where [T; N] : Ord 
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.array.cmp(&other.array)
    }
}


impl<T, I, const N : usize> ArrayWithGenericType<T,N> for NamedVector<T, I, N> where I: ArrayWithGenericType<T,N>
{
    type WithType<T2> = NamedVector<T2,I::WithType<T2>,N>;
}
impl<T, I, const N : usize> ArrayWithGenericSize<T,N> for NamedVector<T, I, N> where I: ArrayWithGenericSize<T,N>
{
    type WithSize<const M:usize> = NamedVector<T,I::WithSize<M>,M>;
}

/*
impl<T, I, const N : usize> std::ops::Add<Self> for NamedVector<T,I,N>
    where T: std::ops::Add<T>, I:ArrayWithGenericType<T,N>
{
    type Output=NamedVector
    <
        <T as ::std::ops::Add<T>>::Output,
        <I as ArrayWithGenericType<T,N>>::WithType<<T as ::std::ops::Add<T>>::Output>,
        N
    >;
    fn add(self, rhs: Self) -> Self::Output { NamedVector::from_array(self.array.map_with(rhs.array, T::add)) }
}
*/




crate::map_on::map_on_operator_binary!(
    (($trait_name: tt, $fn_name: tt)) =>
    {
        impl<T, I, const N: usize> std::ops::$trait_name<Self> for NamedVector<T,I,N>
            where T: std::ops::$trait_name<T>, I:ArrayWithGenericType<T,N>
        {
            type Output=NamedVector
            <
                <T as ::std::ops::$trait_name<T>>::Output,
                <I as ArrayWithGenericType<T,N>>::WithType<<T as ::std::ops::$trait_name<T>>::Output>,
                N
            >;
            fn $fn_name(self, rhs: Self) -> Self::Output { self.array.map_with(rhs.array, T::$fn_name).into() }
        }

        impl<T, I, const N: usize> std::ops::$trait_name<T> for NamedVector<T,I,N> where T: std::ops::$trait_name<T> + Copy
        {
            type Output=NamedVector<<T as ::std::ops::$trait_name<T>>::Output,I,N>;
            fn $fn_name(self, rhs: T) -> Self::Output { self.to_array().map(|v| v.$fn_name(rhs)).into() }
        }
    }
);

*/