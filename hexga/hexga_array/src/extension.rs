pub trait ToArray<T, const N : usize> : Into<[T; N]> { #[inline(always)] fn to_array(self) -> [T; N] { self.into() } }
impl<C, T, const N : usize> ToArray<T, N> for C where C : Into<[T; N]> {}

pub trait FromArray<T, const N : usize> : From<[T; N]> 
{ 
    #[inline(always)] fn from_array(value : [T; N]) -> Self { value.into() } 
    fn from_fn<F>(f : F) -> Self where F : FnMut(usize) -> T { Self::from_array(std::array::from_fn(f)) }
    fn splat(val : T) -> Self where T: Clone { Self::from_fn(|_| val.clone()) }
}
impl<C, T, const N : usize> FromArray<T, N> for C where C : From<[T; N]> {}

pub trait AsArray<T, const N : usize> : AsRef<[T; N]> { #[inline(always)] fn as_array(&self) -> &[T; N] { self.as_ref() } }
impl<C, T, const N : usize> AsArray<T, N> for C where C : AsRef<[T; N]> {}

pub trait AsArrayMut<T, const N : usize> : AsMut<[T; N]> { #[inline(always)] fn as_array_mut(&mut self) -> &mut [T; N] { self.as_mut() } }
impl<C, T, const N : usize> AsArrayMut<T, N> for C where C : AsMut<[T; N]> {}

pub trait AsSlice<T> : AsRef<[T]> { #[inline(always)] fn as_slice(&self) -> &[T] { self.as_ref() } }
impl<C, T> AsSlice<T> for C where C : AsRef<[T]> {}

pub trait AsMutSlice<T> : AsMut<[T]> { #[inline(always)] fn as_mut_slice(&mut self) -> &mut [T] { self.as_mut() } }
impl<C, T> AsMutSlice<T> for C where C : AsMut<[T]> {}




pub trait ToArray1<T> : Into<[T; 1]> { #[inline(always)] fn to_array1(self) -> [T; 1] { self.into() } }
impl<C, T> ToArray1<T> for C where C : Into<[T; 1]> {}

pub trait ToArray2<T> : Into<[T; 2]> { #[inline(always)] fn to_array2(self) -> [T; 2] { self.into() } }
impl<C, T> ToArray2<T> for C where C : Into<[T; 2]> {}

pub trait ToArray3<T> : Into<[T; 3]> { #[inline(always)] fn to_array3(self) -> [T; 3] { self.into() } }
impl<C, T> ToArray3<T> for C where C : Into<[T; 3]> {}

pub trait ToArray4<T> : Into<[T; 4]> { #[inline(always)] fn to_array4(self) -> [T; 4] { self.into() } }
impl<C, T> ToArray4<T> for C where C : Into<[T; 4]> {}

pub trait ToArray5<T> : Into<[T; 5]> { #[inline(always)] fn to_array5(self) -> [T; 5] { self.into() } }
impl<C, T> ToArray5<T> for C where C : Into<[T; 5]> {}

pub trait ToArray6<T> : Into<[T; 6]> { #[inline(always)] fn to_array6(self) -> [T; 6] { self.into() } }
impl<C, T> ToArray6<T> for C where C : Into<[T; 6]> {}

pub trait ToArray7<T> : Into<[T; 7]> { #[inline(always)] fn to_array7(self) -> [T; 7] { self.into() } }
impl<C, T> ToArray7<T> for C where C : Into<[T; 7]> {}

pub trait ToArray8<T> : Into<[T; 8]> { #[inline(always)] fn to_array8(self) -> [T; 8] { self.into() } }
impl<C, T> ToArray8<T> for C where C : Into<[T; 8]> {}



pub trait ArrayToTuple
{
    type Tuple;
    fn to_tuple(self) -> Self::Tuple;
    fn from_tuple(value : Self::Tuple) -> Self;
}

impl<T> ArrayToTuple for [T; 0]
{
    type Tuple=();
    fn to_tuple(self) -> Self::Tuple { }
    fn from_tuple(_ : Self::Tuple) -> Self { [] }
}

impl<T> ArrayToTuple for [T; 1]
{
    type Tuple=(T,);
    fn to_tuple(self) -> Self::Tuple { let [a] = self; (a,) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0] }
}

impl<T> ArrayToTuple for [T; 2]
{
    type Tuple=(T,T,);
    fn to_tuple(self) -> Self::Tuple { let [a, b] = self; (a,b) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0, t.1] }
}

impl<T> ArrayToTuple for [T; 3]
{
    type Tuple=(T,T,T,);
    fn to_tuple(self) -> Self::Tuple { let [a, b, c] = self; (a,b,c) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0, t.1, t.2] }
}

impl<T> ArrayToTuple for [T; 4]
{
    type Tuple=(T,T,T,T,);
    fn to_tuple(self) -> Self::Tuple { let [a, b, c,d] = self; (a,b,c,d) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0, t.1, t.2,t.3] }
}

impl<T> ArrayToTuple for [T; 5]
{
    type Tuple=(T,T,T,T,T,);
    fn to_tuple(self) -> Self::Tuple { let [a, b, c,d, e] = self; (a,b,c,d,e) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0, t.1, t.2,t.3, t.4] }
}

impl<T> ArrayToTuple for [T; 6]
{
    type Tuple=(T,T,T,T,T,T,);
    fn to_tuple(self) -> Self::Tuple { let [a, b, c,d, e,f] = self; (a,b,c,d,e,f) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0, t.1, t.2,t.3, t.4, t.5] }
}

impl<T> ArrayToTuple for [T; 7]
{
    type Tuple=(T,T,T,T,T,T,T,);
    fn to_tuple(self) -> Self::Tuple { let [a, b, c,d, e,f, g] = self; (a,b,c,d,e,f,g) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0, t.1, t.2,t.3, t.4, t.5, t.6] }
}

impl<T> ArrayToTuple for [T; 8]
{
    type Tuple=(T,T,T,T,T,T,T,T,);
    fn to_tuple(self) -> Self::Tuple { let [a, b, c,d, e,f, g, h] = self; (a,b,c,d,e,f,g,h) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0, t.1, t.2,t.3, t.4, t.5, t.6, t.7] }
}
