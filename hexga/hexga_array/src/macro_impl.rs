
// Fixed Binary Operator 
#[macro_export]
macro_rules! impl_fixed_array_like_op_binary
{
    ($name: ident, $dim : expr, $op_trait_name: ident, $op_trait_fn_name : ident) =>
    {
        impl_fixed_array_like_op_binary_composite!($name,$dim,$op_trait_name,$op_trait_fn_name);
        impl_fixed_array_like_op_binary_non_composite!($name,$dim,$op_trait_name,$op_trait_fn_name);
    }
}

#[macro_export]
macro_rules! impl_fixed_array_like_op_binary_composite
{
    ($name: ident, $dim : expr, $op_trait_name: ident, $op_trait_fn_name : ident) =>
    {
        impl<T> $op_trait_name<Self> for $name<T> where T : $op_trait_name<T>
        {
            type Output=$name<<T as $op_trait_name<T>>::Output>;
            fn $op_trait_fn_name(self, rhs: Self) -> Self::Output { self.map_with(rhs, T::$op_trait_fn_name) }
        }
    };
}

#[macro_export]
macro_rules! impl_fixed_array_like_op_binary_non_composite
{
    ($name: ident, $dim : expr, $op_trait_name: ident, $op_trait_fn_name : ident) =>
    {
        // Currently binary op are closed (only work with the same type), they are not like
        // impl<T,O> $op_trait_name<T> for $name<T> where T : $op_trait_name<O> + Copy
        // because this cause conflict of implementation, especially in the case of matrix multiplication by 
        // a scalar T, a Vector<T,N>, another Matrix with the good size
        impl<T> $op_trait_name<T> for $name<T> where T : $op_trait_name<T> + Copy
        {
            type Output=$name<<T as $op_trait_name<T>>::Output>;
            fn $op_trait_fn_name(self, rhs: T) -> Self::Output { <[T;$dim]>::from(self).map(|v| v.$op_trait_fn_name(rhs)).into() }
        }
    };
}


// Generic Binary Operator 
#[macro_export]
macro_rules! impl_generic_array_like_op_binary
{
    ($name: ident, $op_trait_name: ident, $op_trait_fn_name : ident) =>
    {
        impl_generic_array_like_op_binary_composite!($name,$op_trait_name,$op_trait_fn_name);
        impl_generic_array_like_op_binary_non_composite!($name,$op_trait_name,$op_trait_fn_name);
    }
}

#[macro_export]
macro_rules! impl_generic_array_like_op_binary_composite
{
    ($name: ident, $op_trait_name: ident, $op_trait_fn_name : ident) =>
    {
        impl<T, const N : usize> $op_trait_name<Self> for $name<T, N> 
            where T : $op_trait_name<T>
        {
            type Output=$name<<T as $op_trait_name<T>>::Output, N>;
            fn $op_trait_fn_name(self, rhs: Self) -> Self::Output { self.map_with(rhs, T::$op_trait_fn_name) }
        }
    }
}

#[macro_export]
macro_rules! impl_generic_array_like_op_binary_non_composite
{
    ($name: ident, $op_trait_name: ident, $op_trait_fn_name : ident) =>
    {
        impl<T, const N : usize> $op_trait_name<T> for $name<T,N> where T : $op_trait_name<T> + Copy
        {
            type Output=$name<<T as $op_trait_name<T>>::Output,N>;
            fn $op_trait_fn_name(self, rhs: T) -> Self::Output { self.to_array().map(|v| v.$op_trait_fn_name(rhs)).into() }
        }
    }
}


#[macro_export]
macro_rules! impl_fixed_array_like_op_assign
{
    ($name: ident, $dim : expr, $op_trait_name: ident, $op_trait_fn_name : ident) =>
    {
        impl<T> $op_trait_name<Self> for $name<T> where T : $op_trait_name
        {
            fn $op_trait_fn_name(&mut self, rhs: Self) 
            {
                let arr : [T; $dim] = rhs.into();
                self.array_mut().iter_mut().zip(arr.into_iter()).for_each(|(a, b)| a.$op_trait_fn_name(b));
            }
        }

        impl<T> $op_trait_name<&Self> for $name<T> where T : $op_trait_name + Copy
        {
            fn $op_trait_fn_name(&mut self, rhs: &Self) { $op_trait_name::$op_trait_fn_name(self,*rhs) }
        }
    }
}

#[macro_export]
macro_rules! impl_generic_array_like_op_assign
{
    ($name: ident, $op_trait_name: ident, $op_trait_fn_name : ident) =>
    {
        impl<T, const N : usize> $op_trait_name<Self> for $name<T,N> where T : $op_trait_name
        {
            fn $op_trait_fn_name(&mut self, rhs: Self) 
            {
                let arr : [T; N] = rhs.into();
                self.array_mut().iter_mut().zip(arr.into_iter()).for_each(|(a, b)| a.$op_trait_fn_name(b));
            }
        }

        impl<T, const N : usize> $op_trait_name<&Self> for $name<T,N> where T : $op_trait_name + Copy
        {
            fn $op_trait_fn_name(&mut self, rhs: &Self) { $op_trait_name::$op_trait_fn_name(self,*rhs) }
        }
    }
}

#[macro_export]
macro_rules! impl_fixed_array_like_op
{
    ($name: ident, $dim : expr) => 
    {
        impl_fixed_array_like_op_binary!($name, $dim, Add, add);
        impl_fixed_array_like_op_assign!($name, $dim, AddAssign, add_assign);

        impl_fixed_array_like_op_binary!($name, $dim, Sub, sub);
        impl_fixed_array_like_op_assign!($name, $dim, SubAssign, sub_assign);

        impl_fixed_array_like_op_binary!($name, $dim, Mul, mul);
        impl_fixed_array_like_op_assign!($name, $dim, MulAssign, mul_assign);

        impl_fixed_array_like_op_binary!($name, $dim, Div, div);
        impl_fixed_array_like_op_assign!($name, $dim, DivAssign, div_assign);

        impl_fixed_array_like_op_binary!($name, $dim, Rem, rem);
        impl_fixed_array_like_op_assign!($name, $dim, RemAssign, rem_assign);

        impl_fixed_array_like_op_binary!($name, $dim, BitAnd, bitand);
        impl_fixed_array_like_op_assign!($name, $dim, BitAndAssign, bitand_assign);

        impl_fixed_array_like_op_binary!($name, $dim, BitOr, bitor);
        impl_fixed_array_like_op_assign!($name, $dim, BitOrAssign, bitor_assign);

        impl_fixed_array_like_op_binary!($name, $dim, BitXor, bitxor);
        impl_fixed_array_like_op_assign!($name, $dim, BitXorAssign, bitxor_assign);

        impl_fixed_array_like_op_binary!($name, $dim, Shr, shr);
        impl_fixed_array_like_op_assign!($name, $dim, ShrAssign, shr_assign);

        impl_fixed_array_like_op_binary!($name, $dim, Shl, shl);
        impl_fixed_array_like_op_assign!($name, $dim, ShlAssign, shl_assign);
        
        // ================= Unary =========

        impl<T> ::std::ops::Not for $name<T> where T : Not
        {
            type Output = $name<T::Output>;
            fn not(self) -> Self::Output { self.map(|v| v.not()) }
        }

        impl<T> ::std::ops::Neg for $name<T> where T : Neg
        {
            type Output = $name<T::Output>;
            fn neg(self) -> Self::Output { self.map(|v| v.neg()) }
        }

        // ================= Iter =========

        impl<T> ::std::iter::Sum for $name<T> where Self : Zero + Add<Self,Output = Self>
        {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, Self::add)
            }
        }

        impl<T> ::std::iter::Product for $name<T> where Self : One + Mul<Self,Output = Self>
        {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ONE, Self::mul)
            }
        }

        // ================= Display =========

        impl_fixed_array_like_display!($name, Display);
        impl_fixed_array_like_display!($name, Debug);
        impl_fixed_array_like_display!($name, Octal);
        impl_fixed_array_like_display!($name, Binary);
        impl_fixed_array_like_display!($name, LowerHex);
        impl_fixed_array_like_display!($name, UpperHex);
        impl_fixed_array_like_display!($name, LowerExp);
        impl_fixed_array_like_display!($name, UpperExp);
        impl_fixed_array_like_display!($name, Pointer);
    }
}

#[macro_export]
macro_rules! impl_generic_array_like_display
{
    ($name: ident, $trait_name :ident) =>
    {
        impl<T, const N : usize> std::fmt::$trait_name  for $name<T,N> where T : std::fmt::$trait_name  
        { 
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
            {
                write!(f, "(")?;
                let mut it = self.array().iter().peekable();
                while let Some(v) = it.next()
                {
                    v.fmt(f)?;
                    if it.peek().is_some()
                    {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

#[macro_export]
macro_rules! impl_fixed_array_like_display
{
    ($name: ident, $trait_name :ident) =>
    {
        impl<T> std::fmt::$trait_name  for $name<T> where T : std::fmt::$trait_name  
        { 
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
            {
                write!(f, "(")?;
                let mut it = self.array().iter().peekable();
                while let Some(v) = it.next()
                {
                    v.fmt(f)?;
                    if it.peek().is_some()
                    {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

#[macro_export]
macro_rules! impl_generic_array_like_op
{
    ($name: ident) => 
    {
        impl_generic_array_like_op_binary!($name, Add, add);
        impl_generic_array_like_op_assign!($name, AddAssign, add_assign);

        impl_generic_array_like_op_binary!($name, Sub, sub);
        impl_generic_array_like_op_assign!($name, SubAssign, sub_assign);

        impl_generic_array_like_op_binary!($name, Mul, mul);
        impl_generic_array_like_op_assign!($name, MulAssign, mul_assign);

        impl_generic_array_like_op_binary!($name, Div, div);
        impl_generic_array_like_op_assign!($name, DivAssign, div_assign);

        impl_generic_array_like_op_binary!($name, Rem, rem);
        impl_generic_array_like_op_assign!($name, RemAssign, rem_assign);

        impl_generic_array_like_op_binary!($name, BitAnd, bitand);
        impl_generic_array_like_op_assign!($name, BitAndAssign, bitand_assign);

        impl_generic_array_like_op_binary!($name, BitOr, bitor);
        impl_generic_array_like_op_assign!($name, BitOrAssign, bitor_assign);

        impl_generic_array_like_op_binary!($name, BitXor, bitxor);
        impl_generic_array_like_op_assign!($name, BitXorAssign, bitxor_assign);

        impl_generic_array_like_op_binary!($name, Shr, shr);
        impl_generic_array_like_op_assign!($name, ShrAssign, shr_assign);

        impl_generic_array_like_op_binary!($name, Shl, shl);
        impl_generic_array_like_op_assign!($name, ShlAssign, shl_assign);

        // ================= Unary =========

        impl<T, const N : usize> ::std::ops::Not for $name<T,N> where T : Not
        {
            type Output = $name<T::Output,N>;
            fn not(self) -> Self::Output { self.map(|v| v.not()) }
        }

        impl<T, const N : usize> ::std::ops::Neg for $name<T,N> where T : Neg
        {
            type Output = $name<T::Output,N>;
            fn neg(self) -> Self::Output { self.map(|v| v.neg()) }
        }

        // ================= Iter =========
        
        impl<T, const N : usize> ::std::iter::Sum for $name<T,N> where Self : ::hexga_number::Zero + ::std::ops::Add<Self,Output = Self>
        {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, Self::add)
            }
        }

        impl<T, const N : usize> ::std::iter::Product for $name<T,N> where Self : ::hexga_number::One + ::std::ops::Mul<Self,Output = Self>
        {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ONE, Self::mul)
            }
        }

        // ================= Display =========

        impl_generic_array_like_display!($name, Display);
        impl_generic_array_like_display!($name, Debug);
        impl_generic_array_like_display!($name, Octal);
        impl_generic_array_like_display!($name, Binary);
        impl_generic_array_like_display!($name, LowerHex);
        impl_generic_array_like_display!($name, UpperHex);
        impl_generic_array_like_display!($name, LowerExp);
        impl_generic_array_like_display!($name, UpperExp);
        impl_generic_array_like_display!($name, Pointer);
    }
}

#[macro_export]
macro_rules! impl_fixed_array_like
{
    ($name: ident, $dim : expr) => 
    { 
        impl<T> ::std::marker::Copy  for $name<T> where T : Copy {}
        impl<T> ::std::clone::Clone for $name<T> where T : Clone
        {
            fn clone(&self) -> Self { self.array().clone().into() }
        }

        impl<T> ::std::cmp::PartialEq for $name<T> where T : PartialEq
        {
            fn eq(&self, rhs : &Self) -> bool { self.array() == rhs.array() }
        }
        impl<T> ::std::cmp::Eq for $name<T> where T : Eq {}

        impl<T> ::std::hash::Hash for $name<T> where T : Hash
        {
            fn hash<H>(&self, state: &mut H) where H: Hasher { self.as_ref().hash(state); }
        }

        impl<T> ::std::convert::From<T> for $name<T> where T : Copy { fn from(value: T) -> Self { Self::from([value; $dim]) } }
    
        impl<T> ::std::convert::From<[T; $dim]> for $name<T> { fn from(value: [T; $dim]) -> Self { unsafe { std::mem::transmute_copy(&value) } } }
        impl<T> ::std::convert::From<$name<T>> for [T; $dim] { fn from(value: $name<T>) -> Self { unsafe { std::mem::transmute_copy(&value) } } }

        impl<T> ::std::convert::AsRef<[T; $dim]> for $name<T> { fn as_ref(&self) -> &[T; $dim] { unsafe { std::mem::transmute(self) } } }
        impl<T> ::std::convert::AsMut<[T; $dim]> for $name<T> { fn as_mut(&mut self) -> &mut [T; $dim] { unsafe { std::mem::transmute(self) } } }

        impl<T> $crate::ArrayLike<T, $dim> for $name<T>
        {
            type WithType<T2>=$name<T2>;

            fn array(&self) -> &[T; $dim] { unsafe { std::mem::transmute(self) } }
            fn array_mut(&mut self) -> &mut[T; $dim] { unsafe { std::mem::transmute(self) } }
        }

        impl<T, Idx> ::std::ops::Index<Idx> for $name<T> where [T;$dim] : ::std::ops::Index<Idx>
        { 
            type Output=<[T;$dim] as ::std::ops::Index<Idx>>::Output; 
            fn index(&self, index: Idx) -> &Self::Output { self.array().index(index) } 
        }
        impl<T, Idx> ::std::ops::IndexMut<Idx> for $name<T> where [T;$dim] : ::std::ops::IndexMut<Idx>
        { 
            fn index_mut(&mut self, index: Idx) -> &mut Self::Output { self.array_mut().index_mut(index) } 
        }
    
        impl<T, Idx> ::hexga_core::collections::Get<Idx> for $name<T> where [T;$dim] : ::hexga_core::collections::Get<Idx>
        { 
            type Output = <[T;$dim] as ::hexga_core::collections::Get<Idx>>::Output;

            #[inline(always)]
            fn try_get(&self, index: Idx) -> Result<&Self::Output, ()> { ::hexga_core::collections::Get::try_get(self.array(), index) }
            #[inline(always)]
            fn get(&self, index: Idx) -> Option<&Self::Output> { ::hexga_core::collections::Get::get(self.array(), index) } 
            #[inline(always)]
            #[track_caller]
            unsafe fn get_unchecked(&self, index: Idx) -> &Self::Output { unsafe { ::hexga_core::collections::Get::get_unchecked(self.array(), index) } } 
        }
        impl<T, Idx> ::hexga_core::collections::GetMut<Idx> for $name<T> where [T;$dim] : ::hexga_core::collections::GetMut<Idx>
        { 
            #[inline(always)]
            fn try_get_mut(&mut self, index: Idx) -> Result<&mut Self::Output, ()> { ::hexga_core::collections::GetMut::try_get_mut(self.array_mut(), index) } 
            #[inline(always)]
            fn get_mut(&mut self, index: Idx) -> Option<&mut Self::Output> { ::hexga_core::collections::GetMut::get_mut(self.array_mut(), index) } 
            #[inline(always)]
            #[track_caller]
            unsafe fn get_unchecked_mut(&mut self, index: Idx) -> &mut Self::Output { unsafe { ::hexga_core::collections::GetMut::get_unchecked_mut(self.array_mut(), index) } } 
        }

        impl<T, Idx> ::hexga_core::collections::GetManyMut<Idx> for $name<T> where [T;$dim] : ::hexga_core::collections::GetManyMut<Idx>
        {
            #[inline(always)]
            fn try_get_disjoint_mut<const N: usize>(&mut self, indices: [Idx; N]) -> Result<[&mut Self::Output;N], ()> { ::hexga_core::collections::GetManyMut::try_get_disjoint_mut(self.array_mut(), indices) }
            #[inline(always)]
            #[track_caller]
            unsafe fn get_disjoint_unchecked_mut<const N: usize>(&mut self, indices: [Idx; N]) -> [&mut Self::Output;N] { unsafe { ::hexga_core::collections::GetManyMut::get_disjoint_unchecked_mut(self.array_mut(), indices) } }
        }

        impl<T> ::std::iter::IntoIterator for $name<T> where [T;$dim] : ::std::iter::IntoIterator
        {
            type Item = <[T;$dim] as ::std::iter::IntoIterator>::Item;
            type IntoIter = <[T;$dim] as ::std::iter::IntoIterator>::IntoIter;
        
            fn into_iter(self) -> Self::IntoIter 
            {
                self.to_array().into_iter()
            }
        }
        
        impl<'a, T> ::std::iter::IntoIterator for &'a $name<T> where &'a [T;$dim] : ::std::iter::IntoIterator
        {
            type Item = <&'a [T;$dim] as ::std::iter::IntoIterator>::Item;
            type IntoIter = <&'a [T;$dim] as ::std::iter::IntoIterator>::IntoIter;
        
            fn into_iter(self) -> Self::IntoIter {
                self.as_array().into_iter()
            }
        }
        
        impl<'a, T> ::std::iter::IntoIterator for &'a mut $name<T> where &'a mut [T;$dim] : ::std::iter::IntoIterator
        {
            type Item = <&'a mut [T;$dim] as IntoIterator>::Item;
            type IntoIter = <&'a mut [T;$dim] as IntoIterator>::IntoIter;
        
            fn into_iter(self) -> Self::IntoIter {
                self.as_array_mut().into_iter()
            }
        }

        #[cfg(feature = "serde")]
        impl<T> ::serde::Serialize for $name<T> where [T;$dim] : ::serde::Serialize
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer,
            { self.as_array().serialize(serializer) }
        }

        #[cfg(feature = "serde")]
        impl<'de, T> ::serde::Deserialize<'de> for $name<T> where [T;$dim] : ::serde::Deserialize<'de>{
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de>,
            {
                Ok(<[T;$dim]>::deserialize(deserializer)?.into())
            }
        }
    };
}



#[macro_export]
macro_rules! impl_fixed_array_like_with_op
{
    ($name: ident, $dim : expr) => 
    { 
        impl_fixed_array_like_op!($name, $dim);
        impl_fixed_array_like!($name, $dim);
    };
}



#[macro_export]
macro_rules! impl_generic_array_like
{
    ($name: ident) => 
    { 
        impl<T, const N : usize> ::std::marker::Copy  for $name<T,N> where T : Copy  {}
        impl<T, const N : usize> ::std::clone::Clone for $name<T,N> where T : Clone
        {
            fn clone(&self) -> Self { self.array().clone().into() }
        }

        impl<T, const N : usize> ::std::cmp::PartialEq for $name<T,N> where T : PartialEq
        {
            fn eq(&self, rhs : &Self) -> bool { self.array() == rhs.array() }
        }
        impl<T, const N : usize> ::std::cmp::Eq for $name<T,N> where T : Eq{}

        impl<T, const N : usize> ::std::hash::Hash for $name<T,N> where T : Hash
        {
            fn hash<H>(&self, state: &mut H) where H: Hasher { self.as_ref().hash(state); }
        }

        impl<T, const N : usize> ::std::convert::From<T> for $name<T,N> where T : Copy { fn from(value: T) -> Self { Self::from([value; N]) } }

        impl<T, const N : usize> ::std::convert::From<[T; N]> for $name<T,N> { fn from(value: [T; N]) -> Self { unsafe { std::mem::transmute_copy(&value) } } }
        impl<T, const N : usize> ::std::convert::From<$name<T,N>> for [T; N] { fn from(value: $name<T,N>) -> Self { unsafe { std::mem::transmute_copy(&value) } } }

        impl<T, const N : usize> ::std::convert::AsRef<[T; N]> for $name<T,N> { fn as_ref(&self) -> &[T; N] { unsafe { std::mem::transmute(self) } } }
        impl<T, const N : usize> ::std::convert::AsMut<[T; N]> for $name<T,N> { fn as_mut(&mut self) -> &mut [T; N] { unsafe { std::mem::transmute(self) } } }

        impl<T, const N : usize> $crate::ArrayLike<T, N> for $name<T,N>
        {
            type WithType<T2>=$name<T2,N>;

            fn array(&self) -> &[T; N] { unsafe { std::mem::transmute(self) } }
            fn array_mut(&mut self) -> &mut[T; N] { unsafe { std::mem::transmute(self) } }
        }

        impl<T, const N : usize, Idx> ::std::ops::Index<Idx> for $name<T,N> where [T;N] : ::std::ops::Index<Idx>
        { 
            type Output=<[T;N] as ::std::ops::Index<Idx>>::Output; 
            fn index(&self, index: Idx) -> &Self::Output { self.array().index(index) } 
        }
        impl<T, const N : usize, Idx> ::std::ops::IndexMut<Idx> for $name<T,N> where [T;N] : ::std::ops::IndexMut<Idx>
        { 
            fn index_mut(&mut self, index: Idx) -> &mut Self::Output { self.array_mut().index_mut(index) } 
        }
    
        impl<T, const N : usize, Idx> ::hexga_core::collections::Get<Idx> for $name<T,N> where [T;N] : ::hexga_core::collections::Get<Idx>
        { 
            type Output = <[T;N] as ::hexga_core::collections::Get<Idx>>::Output;

            #[inline(always)]
            fn try_get(&self, index: Idx) -> Result<&Self::Output, ()> { ::hexga_core::collections::Get::try_get(self.array(), index) }
            #[inline(always)]
            fn get(&self, index: Idx) -> Option<&Self::Output> { ::hexga_core::collections::Get::get(self.array(), index) } 
            #[inline(always)]
            #[track_caller]
            unsafe fn get_unchecked(&self, index: Idx) -> &Self::Output { unsafe { ::hexga_core::collections::Get::get_unchecked(self.array(), index) } } 
        }
        impl<T, const N : usize, Idx> ::hexga_core::collections::GetMut<Idx> for $name<T,N> where [T;N] : ::hexga_core::collections::GetMut<Idx>
        { 
            #[inline(always)]
            fn try_get_mut(&mut self, index: Idx) -> Result<&mut Self::Output, ()> { ::hexga_core::collections::GetMut::try_get_mut(self.array_mut(), index) } 
            #[inline(always)]
            fn get_mut(&mut self, index: Idx) -> Option<&mut Self::Output> { ::hexga_core::collections::GetMut::get_mut(self.array_mut(), index) } 
            #[inline(always)]
            #[track_caller]
            unsafe fn get_unchecked_mut(&mut self, index: Idx) -> &mut Self::Output { unsafe { ::hexga_core::collections::GetMut::get_unchecked_mut(self.array_mut(), index) } } 
        }

        impl<T, const N : usize, Idx> ::hexga_core::collections::GetManyMut<Idx> for $name<T,N> where [T;N] : ::hexga_core::collections::GetManyMut<Idx>
        {
            #[inline(always)]
            fn try_get_disjoint_mut<const N2: usize>(&mut self, indices: [Idx; N2]) -> Result<[&mut Self::Output;N2], ()> { ::hexga_core::collections::GetManyMut::try_get_disjoint_mut(self.array_mut(), indices) }
            #[inline(always)]
            #[track_caller]
            unsafe fn get_disjoint_unchecked_mut<const N2: usize>(&mut self, indices: [Idx; N2]) -> [&mut Self::Output;N2] { unsafe { ::hexga_core::collections::GetManyMut::get_disjoint_unchecked_mut(self.array_mut(), indices) } }
        }

        impl<T, const N : usize> ::std::iter::IntoIterator for $name<T,N> where [T;N] : ::std::iter::IntoIterator
        {
            type Item = <[T;N] as ::std::iter::IntoIterator>::Item;
            type IntoIter = <[T;N] as ::std::iter::IntoIterator>::IntoIter;
        
            fn into_iter(self) -> Self::IntoIter 
            {
                self.to_array().into_iter()
            }
        }
        
        impl<'a, T, const N : usize> ::std::iter::IntoIterator for &'a $name<T,N> where &'a [T;N] : ::std::iter::IntoIterator
        {
            type Item = <&'a [T;N] as ::std::iter::IntoIterator>::Item;
            type IntoIter = <&'a [T;N] as ::std::iter::IntoIterator>::IntoIter;
        
            fn into_iter(self) -> Self::IntoIter {
                self.as_array().into_iter()
            }
        }
        
        impl<'a, T, const N : usize> ::std::iter::IntoIterator for &'a mut $name<T,N> where &'a mut [T;N] : ::std::iter::IntoIterator
        {
            type Item = <&'a mut [T;N] as IntoIterator>::Item;
            type IntoIter = <&'a mut [T;N] as IntoIterator>::IntoIter;
        
            fn into_iter(self) -> Self::IntoIter {
                self.as_array_mut().into_iter()
            }
        }

        #[cfg(feature = "serde")]
        impl<T, const N : usize> ::serde::Serialize for $name<T,N> where [T;N] : ::serde::Serialize
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer,
            { self.as_array().serialize(serializer) }
        }

        #[cfg(feature = "serde")]
        impl<'de, T, const N : usize> ::serde::Deserialize<'de> for $name<T,N> where [T;N] : ::serde::Deserialize<'de>{
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de>,
            {
                Ok(<[T;N]>::deserialize(deserializer)?.into())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_generic_array_like_with_op
{
    ($name: ident) => 
    { 
        impl_generic_array_like_op!($name);
        impl_generic_array_like!($name);
    };
}
