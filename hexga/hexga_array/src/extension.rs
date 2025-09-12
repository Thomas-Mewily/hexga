pub trait ExtensionToArray<T, const N : usize> : Into<[T; N]> { #[inline(always)] fn to_array(self) -> [T; N] { self.into() } }
impl<C, T, const N : usize> ExtensionToArray<T, N> for C where C : Into<[T; N]> {}

pub trait ExtensionFromArray<T, const N : usize> : From<[T; N]> 
{ 
    #[inline(always)] fn from_array(value : [T; N]) -> Self { value.into() } 
    fn from_fn<F>(f : F) -> Self where F : FnMut(usize) -> T { Self::from_array(std::array::from_fn(f)) }
    fn splat(val : T) -> Self where T: Clone { Self::from_fn(|_| val.clone()) }
}
impl<C, T, const N : usize> ExtensionFromArray<T, N> for C where C : From<[T; N]> {}

pub trait ExtensionAsArray<T, const N : usize> : AsRef<[T; N]> { #[inline(always)] fn as_array(&self) -> &[T; N] { self.as_ref() } }
impl<C, T, const N : usize> ExtensionAsArray<T, N> for C where C : AsRef<[T; N]> {}

pub trait ExtensionAsArrayMut<T, const N : usize> : AsMut<[T; N]> { #[inline(always)] fn as_array_mut(&mut self) -> &mut [T; N] { self.as_mut() } }
impl<C, T, const N : usize> ExtensionAsArrayMut<T, N> for C where C : AsMut<[T; N]> {}

pub trait ExtensionAsSlice<T> : AsRef<[T]> { #[inline(always)] fn as_slice(&self) -> &[T] { self.as_ref() } }
impl<C, T> ExtensionAsSlice<T> for C where C : AsRef<[T]> {}

pub trait ExtensionAsMutSlice<T> : AsMut<[T]> { #[inline(always)] fn as_mut_slice(&mut self) -> &mut [T] { self.as_mut() } }
impl<C, T> ExtensionAsMutSlice<T> for C where C : AsMut<[T]> {}




pub trait ExtensionToArray1<T> : Into<[T; 1]> { #[inline(always)] fn to_array1(self) -> [T; 1] { self.into() } }
impl<C, T> ExtensionToArray1<T> for C where C : Into<[T; 1]> {}

pub trait ExtensionToArray2<T> : Into<[T; 2]> { #[inline(always)] fn to_array2(self) -> [T; 2] { self.into() } }
impl<C, T> ExtensionToArray2<T> for C where C : Into<[T; 2]> {}

pub trait ExtensionToArray3<T> : Into<[T; 3]> { #[inline(always)] fn to_array3(self) -> [T; 3] { self.into() } }
impl<C, T> ExtensionToArray3<T> for C where C : Into<[T; 3]> {}

pub trait ExtensionToArray4<T> : Into<[T; 4]> { #[inline(always)] fn to_array4(self) -> [T; 4] { self.into() } }
impl<C, T> ExtensionToArray4<T> for C where C : Into<[T; 4]> {}

pub trait ExtensionToArray5<T> : Into<[T; 5]> { #[inline(always)] fn to_array5(self) -> [T; 5] { self.into() } }
impl<C, T> ExtensionToArray5<T> for C where C : Into<[T; 5]> {}

pub trait ExtensionToArray6<T> : Into<[T; 6]> { #[inline(always)] fn to_array6(self) -> [T; 6] { self.into() } }
impl<C, T> ExtensionToArray6<T> for C where C : Into<[T; 6]> {}

pub trait ExtensionToArray7<T> : Into<[T; 7]> { #[inline(always)] fn to_array7(self) -> [T; 7] { self.into() } }
impl<C, T> ExtensionToArray7<T> for C where C : Into<[T; 7]> {}

pub trait ExtensionToArray8<T> : Into<[T; 8]> { #[inline(always)] fn to_array8(self) -> [T; 8] { self.into() } }
impl<C, T> ExtensionToArray8<T> for C where C : Into<[T; 8]> {}



pub trait ExtensionArrayToTuple
{
    type Tuple;
    fn to_tuple(self) -> Self::Tuple;
    fn from_tuple(value : Self::Tuple) -> Self;
}

impl<T> ExtensionArrayToTuple for [T; 0]
{
    type Tuple=();
    fn to_tuple(self) -> Self::Tuple { }
    fn from_tuple(_ : Self::Tuple) -> Self { [] }
}

impl<T> ExtensionArrayToTuple for [T; 1]
{
    type Tuple=(T,);
    fn to_tuple(self) -> Self::Tuple { let [a] = self; (a,) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0] }
}

impl<T> ExtensionArrayToTuple for [T; 2]
{
    type Tuple=(T,T,);
    fn to_tuple(self) -> Self::Tuple { let [a, b] = self; (a,b) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0, t.1] }
}

impl<T> ExtensionArrayToTuple for [T; 3]
{
    type Tuple=(T,T,T,);
    fn to_tuple(self) -> Self::Tuple { let [a, b, c] = self; (a,b,c) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0, t.1, t.2] }
}

impl<T> ExtensionArrayToTuple for [T; 4]
{
    type Tuple=(T,T,T,T,);
    fn to_tuple(self) -> Self::Tuple { let [a, b, c,d] = self; (a,b,c,d) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0, t.1, t.2,t.3] }
}

impl<T> ExtensionArrayToTuple for [T; 5]
{
    type Tuple=(T,T,T,T,T,);
    fn to_tuple(self) -> Self::Tuple { let [a, b, c,d, e] = self; (a,b,c,d,e) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0, t.1, t.2,t.3, t.4] }
}

impl<T> ExtensionArrayToTuple for [T; 6]
{
    type Tuple=(T,T,T,T,T,T,);
    fn to_tuple(self) -> Self::Tuple { let [a, b, c,d, e,f] = self; (a,b,c,d,e,f) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0, t.1, t.2,t.3, t.4, t.5] }
}

impl<T> ExtensionArrayToTuple for [T; 7]
{
    type Tuple=(T,T,T,T,T,T,T,);
    fn to_tuple(self) -> Self::Tuple { let [a, b, c,d, e,f, g] = self; (a,b,c,d,e,f,g) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0, t.1, t.2,t.3, t.4, t.5, t.6] }
}

impl<T> ExtensionArrayToTuple for [T; 8]
{
    type Tuple=(T,T,T,T,T,T,T,T,);
    fn to_tuple(self) -> Self::Tuple { let [a, b, c,d, e,f, g, h] = self; (a,b,c,d,e,f,g,h) }
    fn from_tuple(t : Self::Tuple) -> Self { [t.0, t.1, t.2,t.3, t.4, t.5, t.6, t.7] }
}
