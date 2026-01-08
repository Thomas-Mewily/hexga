use proc_macro::{TokenStream};
use proc_macro_crate::{FoundCrate, crate_name};
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields, GenericParam, Type, Ident};

/// Derive macro for fixed-size math vectors and structs.
///
/// Supports two kinds of structs:
///
/// 1. **Array-backed structs**:
///    - Wrapper around an array `[T; N]`.
///    - Implements `serde::Serialize` / `Deserialize` as a tuple/sequence.
///
///      ```ignore
///      #[math_vec]
///      pub struct Vector<T, const N: usize> { array: [T; N] }
///
///      let v = Vector { array: [1, 2, 3] };
///      // Serialized as tuple: [1, 2, 3]
///      ```
///
/// 2. **Structure-based structs**:
///    - Each element has a name (e.g., `r, g, b, a` or `physic, magic, melee`).
///    - Implements `serde::Serialize` / `Deserialize` as a struct with named fields.
///    - Missing fields are allowed only if `#[non_exhaustive]` is used; they are automatically initialized with `Zero` (`<T as Zero>::ZERO`).
///    - Examples:
///
///      ```ignore
///      #[math_vec]
///      pub struct Color<T> { r: T, g: T, b: T, a: T }
///      // Serialized as struct: { "r": 255, "g": 127, "b": 0, "a": 255 }
///
///      #[math_vec]
///      #[non_exhaustive]
///      pub struct Damage<T> { physic: T, magic: T, melee: T }
///      // Serialized as struct: { "physic": 10, "magic": 5, "melee": 0 }
///      // Missing fields during deserialization get Zero
///      ```
#[proc_macro_attribute]
pub fn math_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let name = &input.ident;

    let hexga_math_crate = crate_name("hexga_math")
        .unwrap_or_else(|_| FoundCrate::Name("hexga_math".to_string()));

    let crate_name_str = match hexga_math_crate {
        FoundCrate::Itself => "crate",
        FoundCrate::Name(ref name) => name,
    };

    let crate_ident = Ident::new(crate_name_str, proc_macro2::Span::call_site());


    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();

    let has_non_exhaustive = input.attrs.iter().any(|a| {
        a.path().is_ident("non_exhaustive")
    });

    // Detect const generic
    let const_generic = input.generics.params.iter().find_map(|p| {
        if let GenericParam::Const(c) = p {
            Some(&c.ident)
        } else {
            None
        }
    });


    let dim = match (&input.fields, const_generic) {
        (Fields::Named(fields), None) => {
            let len = fields.named.len();
            quote! { #len }
        }

        (Fields::Named(fields), Some(_n)) => {
            if fields.named.len() != 1 {
                return syn::Error::new_spanned(
                    fields,
                    "math_vec with const generic requires exactly one array field",
                )
                .to_compile_error()
                .into();
            }

            let field = fields.named.iter().next().unwrap();
            match &field.ty {
                Type::Array(arr) => {
                    let len = &arr.len;
                    quote! { #len }
                }
                _ => {
                    return syn::Error::new_spanned(
                        &field.ty,
                        "expected an array field `[T; N]`",
                    )
                    .to_compile_error()
                    .into();
                }
            }
        }

        _ => {
            return syn::Error::new_spanned(
                &input,
                "math_vec only supports structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    };


    let name_coef_str = format!("{}Coef", &input.ident);
    let name_coef = Ident::new(&name_coef_str, name.span());


    let is_array_struct = input.generics.params.iter().any(|p| matches!(p, syn::GenericParam::Const(_)));

    let names = [&name, &name_coef];

    let [array_impl, array_impl_coef, specific_impl] =
    if is_array_struct
    {
        std::array::from_fn(|idx|
            {
                if idx <= 1
                {
                    let current_name = names[idx];

                    quote! {
                        impl<T, const N: usize> #crate_ident::array::ArrayWithType<T, N> for #current_name<T, N>
                        {
                            type WithType<T2> = #current_name<T2, N>;
                        }
                        impl<T, const N : usize> #crate_ident::array::ArrayWithSize<T, N> for #current_name<T,N>
                        {
                            type WithSize<const M:usize>=#current_name<T,M>;
                        }

                        impl<T, const N : usize, Idx> ::std::ops::Index<Idx> for #current_name<T,N> where [T;N] : ::std::ops::Index<Idx>
                        {
                            type Output=<[T;N] as ::std::ops::Index<Idx>>::Output;
                            #[inline(always)]
                            fn index(&self, index: Idx) -> &Self::Output { self.array().index(index) }
                        }
                        impl<T, const N : usize, Idx> ::std::ops::IndexMut<Idx> for #current_name<T,N> where [T;N] : ::std::ops::IndexMut<Idx>
                        {
                            #[inline(always)]
                            fn index_mut(&mut self, index: Idx) -> &mut Self::Output { self.array_mut().index_mut(index) }
                        }


                        impl<T, const N : usize, Idx> #crate_ident::hexga_core::collections::Get<Idx> for #current_name<T,N> where [T;N] : #crate_ident::hexga_core::collections::Get<Idx>
                        {
                            type Output = <[T;N] as #crate_ident::hexga_core::collections::Get<Idx>>::Output;

                            #[inline(always)]
                            fn get(&self, index: Idx) -> Option<&Self::Output> { #crate_ident::hexga_core::collections::Get::get(self.array(), index) }
                            #[inline(always)]
                            #[track_caller]
                            unsafe fn get_unchecked(&self, index: Idx) -> &Self::Output { unsafe { #crate_ident::hexga_core::collections::Get::get_unchecked(self.array(), index) } }
                        }
                        impl<T, const N : usize, Idx> #crate_ident::hexga_core::collections::TryGet<Idx> for #current_name<T,N> where [T;N] : #crate_ident::hexga_core::collections::TryGet<Idx>
                        {
                            type Error = <[T;N] as #crate_ident::hexga_core::collections::TryGet<Idx>>::Error;

                            #[inline(always)]
                            fn try_get(&self, index: Idx) -> Result<&Self::Output, Self::Error>
                            {
                                #crate_ident::hexga_core::collections::TryGet::try_get(self.array(), index)
                            }
                        }
                        impl<T, const N : usize, Idx> #crate_ident::hexga_core::collections::GetMut<Idx> for #current_name<T,N> where [T;N] : #crate_ident::hexga_core::collections::GetMut<Idx>
                        {
                            #[inline(always)]
                            fn get_mut(&mut self, index: Idx) -> Option<&mut Self::Output> { #crate_ident::hexga_core::collections::GetMut::get_mut(self.array_mut(), index) }
                            #[inline(always)]
                            #[track_caller]
                            unsafe fn get_unchecked_mut(&mut self, index: Idx) -> &mut Self::Output { unsafe { #crate_ident::hexga_core::collections::GetMut::get_unchecked_mut(self.array_mut(), index) } }
                        }
                        impl<T, const N : usize, Idx> #crate_ident::hexga_core::collections::TryGetMut<Idx> for #current_name<T,N> where [T;N] : #crate_ident::hexga_core::collections::TryGetMut<Idx>
                        {
                            #[inline(always)]
                            fn try_get_mut(&mut self, index: Idx) -> Result<&mut Self::Output, Self::Error> { #crate_ident::hexga_core::collections::TryGetMut::try_get_mut(self.array_mut(), index) }
                        }


                        impl<T, const N : usize, Idx> #crate_ident::hexga_core::collections::GetManyMut<Idx> for #current_name<T,N> where [T;N] : #crate_ident::hexga_core::collections::GetManyMut<Idx>
                        {
                            #[inline(always)]
                            fn try_get_many_mut<const N2: usize>(&mut self, indices: [Idx; N2]) -> Result<[&mut Self::Output;N2], ManyMutError> { #crate_ident::hexga_core::collections::GetManyMut::try_get_many_mut(self.array_mut(), indices) }
                            #[inline(always)]
                            fn get_many_mut<const N2: usize>(&mut self, indices: [Idx; N2]) -> Option<[&mut Self::Output;N2]> { #crate_ident::hexga_core::collections::GetManyMut::get_many_mut(self.array_mut(), indices) }

                            #[inline(always)]
                            #[track_caller]
                            unsafe fn get_many_unchecked_mut<const N2: usize>(&mut self, indices: [Idx; N2]) -> [&mut Self::Output;N2] { unsafe { #crate_ident::hexga_core::collections::GetManyMut::get_many_unchecked_mut(self.array_mut(), indices) } }
                        }


                        impl<T, const N : usize> ::std::iter::IntoIterator for #current_name<T,N> where [T;N] : ::std::iter::IntoIterator
                        {
                            type Item = <[T;N] as ::std::iter::IntoIterator>::Item;
                            type IntoIter = <[T;N] as ::std::iter::IntoIterator>::IntoIter;

                            fn into_iter(self) -> Self::IntoIter
                            {
                                <[T;N]>::from(self).into_iter()
                            }
                        }

                        impl<'a, T, const N : usize> ::std::iter::IntoIterator for &'a #current_name<T,N> where &'a [T;N] : ::std::iter::IntoIterator
                        {
                            type Item = <&'a [T;N] as ::std::iter::IntoIterator>::Item;
                            type IntoIter = <&'a [T;N] as ::std::iter::IntoIterator>::IntoIter;

                            fn into_iter(self) -> Self::IntoIter
                            {
                                let array : &[T;N] = self.as_ref();
                                array.into_iter()
                            }
                        }

                        impl<'a, T, const N : usize> ::std::iter::IntoIterator for &'a mut #current_name<T,N> where &'a mut [T;N] : ::std::iter::IntoIterator
                        {
                            type Item = <&'a mut [T;N] as IntoIterator>::Item;
                            type IntoIter = <&'a mut [T;N] as IntoIterator>::IntoIter;

                            fn into_iter(self) -> Self::IntoIter {
                                let array : &mut [T;N] = self.as_mut();
                                array.into_iter()
                            }
                        }


                        impl<T, const N : usize> #crate_ident::map::MapIntern for #current_name<T,N>
                        {
                            type Item=T;
                            fn map_intern<F>(self, f: F) -> Self where F: FnMut(Self::Item) -> Self::Item
                            {
                                Self::from_array(<[T;N]>::from(self).map_intern(f))
                            }
                        }
                        impl<T, const N : usize> #crate_ident::map::MapInternWith for #current_name<T,N>
                        {
                            fn map_with_intern<F>(self, other: Self, f: F) -> Self where F: FnMut(Self::Item, Self::Item) -> Self::Item
                            {
                                Self::from_array(<[T;N]>::from(self).map_with_intern(other.into(), f))
                            }
                        }
                        impl<T, const N : usize> #crate_ident::map::Map for #current_name<T,N>
                        {
                            type WithType<T2> = #current_name<T2,N>;
                            fn map<T2,F>(self, f: F) -> Self::WithType<T2> where F: FnMut(Self::Item) -> T2 {
                                Self::WithType::from_array(<[T;N] as Map>::map(self.into(), f))
                            }
                        }
                        impl<T, const N : usize> #crate_ident::map::MapWith for #current_name<T,N>
                        {
                            fn map_with<R,Item2,F>(self, other: Self::WithType<Item2>, f: F) -> Self::WithType<R> where F: FnMut(Self::Item, Item2) -> R
                            {
                                Self::WithType::from_array(<[T;N]>::from(self).map_with(other.into(), f))
                            }
                        }

                        #crate_ident::map_on::map_on_operator_binary!(
                            (($trait_name: tt, $fn_name: tt)) =>
                            {
                                impl<T, const N : usize> std::ops::$trait_name<Self> for #current_name<T,N>
                                    where T: std::ops::$trait_name<T>
                                {
                                    type Output=#current_name<<T as ::std::ops::$trait_name<T>>::Output,N>;
                                    fn $fn_name(self, rhs: Self) -> Self::Output { self.map_with(rhs, T::$fn_name) }
                                }

                                impl<T, const N : usize> std::ops::$trait_name<T> for #current_name<T,N> where T: std::ops::$trait_name<T> + Copy
                                {
                                    type Output=#current_name<<T as ::std::ops::$trait_name<T>>::Output,N>;
                                    fn $fn_name(self, rhs: T) -> Self::Output { self.to_array().map(|v| v.$fn_name(rhs)).into() }
                                }
                            }
                        );

                        #crate_ident::map_on::map_on_operator_assign!(
                            (($trait_name: tt, $fn_name: tt)) =>
                            {
                                impl<T, const N : usize> ::std::ops::$trait_name<Self> for #current_name<T,N> where T: ::std::ops::$trait_name
                                {
                                    fn $fn_name(&mut self, rhs: Self)
                                    {
                                        let arr : [T;N] = rhs.into();
                                        self.array_mut().iter_mut().zip(arr.into_iter()).for_each(|(a, b)| a.$fn_name(b));
                                    }
                                }

                                impl<T, const N : usize> ::std::ops::$trait_name<&Self> for #current_name<T,N> where T: ::std::ops::$trait_name + Copy
                                {
                                    fn $fn_name(&mut self, rhs: &Self) { ::std::ops::$trait_name::$fn_name(self,*rhs) }
                                }
                            }
                        );

                        // ================= Unary =========

                        impl<T, const N : usize> ::std::ops::Not for #current_name<T,N> where T: ::std::ops::Not
                        {
                            type Output = #current_name<T::Output,N>;
                            fn not(self) -> Self::Output { self.map(::std::ops::Not::not) }
                        }

                        impl<T, const N : usize> ::std::ops::Neg for #current_name<T,N> where T: ::std::ops::Neg
                        {
                            type Output = #current_name<T::Output,N>;
                            fn neg(self) -> Self::Output { self.map(::std::ops::Neg::neg) }
                        }

                        // ================= Iter =========

                        impl<T, const N : usize> ::std::iter::Sum for #current_name<T,N> where Self : #crate_ident::number::Zero + ::std::ops::Add<Self,Output = Self>
                        {
                            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                                iter.fold(<Self as #crate_ident::number::Zero>::ZERO, <Self as ::std::ops::Add<Self>>::add)
                            }
                        }

                        impl<T, const N : usize> ::std::iter::Product for #current_name<T,N> where Self : #crate_ident::number::One + ::std::ops::Mul<Self,Output = Self>
                        {
                            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                                iter.fold(<Self as #crate_ident::number::One>::ONE, <Self as ::std::ops::Mul<Self>>::mul)
                            }
                        }

                        unsafe impl<T, const N : usize> #crate_ident::hexga_core::bit::BitAllUsed for #current_name<T,N> where T:BitAllUsed {}
                        unsafe impl<T, const N : usize> #crate_ident::hexga_core::bit::BitZero for #current_name<T,N> where T: BitZero {}
                        impl<T, const N : usize> #crate_ident::hexga_core::bit::BitPattern for #current_name<T,N> where T: BitPattern, [T::Bits;N] : BitAnyPattern
                        {
                            type Bits = [T::Bits;N];

                            fn is_bit_pattern_valid(bits: &Self::Bits) -> bool {
                                bits.iter().all(|b| T::is_bit_pattern_valid(b))
                            }
                        }

                        #crate_ident::map_on::map_on_constant!
                        (
                            (($trait_name: tt, $constant_name: tt)) =>
                            {
                                impl<T, const N:usize> #crate_ident::number::$trait_name for #current_name<T,N> where T: #crate_ident::number::$trait_name + ::std::marker::Copy
                                {
                                    const $constant_name: Self = Self::from_array(<[T;N]>::$constant_name);
                                }
                            }
                        );

                        #crate_ident::map_on::map_on_std_fmt!(
                            ($trait_name :ident) =>
                            {
                                impl<T, const N : usize> std::fmt::$trait_name  for #current_name<T,N> where T: std::fmt::$trait_name
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
                        );
                    }
                }else
                {
                    quote! {
                        #[cfg(feature = "serde")]
                        impl<T, const N: usize> ::serde::Serialize for #name<T, N>
                            where T: ::serde::Serialize,
                        {
                            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                            where
                                S: ::serde::Serializer,
                            {
                                use ::serde::ser::SerializeTuple;

                                let arr = self.as_array();
                                let mut tup = serializer.serialize_tuple(N)?;
                                for item in arr {
                                    tup.serialize_element(item)?;
                                }
                                tup.end()
                            }
                        }

                        #[cfg(feature = "serde")]
                        const _ : () =
                        {
                            // Thank serde for not supporting the deserialization of variadic array...
                            #[derive(Debug)]
                            struct Arr<T, const N: usize>(pub [T;N]);

                            impl<'de, T, const N: usize> ::serde::Deserialize<'de> for #name<T,N>
                                where T: ::serde::Deserialize<'de>
                            {
                                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                                where
                                    D: ::serde::Deserializer<'de>,
                                {
                                    struct ArrVisitor<T, const N: usize>(::std::marker::PhantomData<T>);

                                    impl<'de, T, const N: usize> ::serde::de::Visitor<'de> for ArrVisitor<T, N>
                                    where
                                        T: ::serde::Deserialize<'de>,
                                    {
                                        type Value = Arr<T, N>;

                                        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                            write!(formatter, "an {} of length {}", std::any::type_name::<#name<T,N>>(), N)
                                        }

                                        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                                        where
                                            A: ::serde::de::SeqAccess<'de>,
                                        {
                                            // SAFETY: We'll ensure every element is initialized or properly dropped
                                            let mut data: [::std::mem::MaybeUninit<T>; N] = unsafe { ::std::mem::MaybeUninit::uninit().assume_init() };
                                            for i in 0..N {
                                                match seq.next_element()? {
                                                    Some(value) => data[i] = ::std::mem::MaybeUninit::new(value),
                                                    None => {
                                                        if ::core::mem::needs_drop::<T>() {
                                                            // Drop any already initialized elements before returning
                                                            for j in 0..i {
                                                                unsafe { ::std::ptr::drop_in_place(data[j].as_mut_ptr()) };
                                                            }
                                                        }
                                                        return Err(::serde::de::Error::invalid_length(i, &self));
                                                    }
                                                }
                                            }

                                            // Ensure no extra elements
                                            if seq.next_element::<::serde::de::IgnoredAny>()?.is_some() {
                                                if ::core::mem::needs_drop::<T>() {
                                                    for i in 0..N {
                                                        unsafe { ::core::ptr::drop_in_place(data[i].as_mut_ptr()) };
                                                    }
                                                }
                                                return Err(::serde::de::Error::invalid_length(N + 1, &self));
                                            }

                                            // SAFETY: All elements are initialized
                                            let result = unsafe { ::std::ptr::read(&data as *const _ as *const [T;N]) };
                                            Ok(Arr(result))
                                        }
                                    }

                                    deserializer.deserialize_tuple(N, ArrVisitor::<T, N>(::std::marker::PhantomData)).map(|arr| #name::<T, N>::from(arr.0))
                                }
                            }
                        };

                        #[cfg(feature = "serde")]
                        impl<T, const N: usize> ::serde::Serialize for #name_coef<T, N>
                            where #name<T, N>: ::serde::Serialize,
                        {
                            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                            where
                                S: ::serde::Serializer,
                            {
                                self.value.serialize(serializer)
                            }
                        }

                        #[cfg(feature = "serde")]
                        impl<'de, T, const N: usize> ::serde::Deserialize<'de> for #name_coef<T, N>
                            where #name<T, N>: ::serde::Deserialize<'de>
                        {
                            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                            where
                                D: ::serde::Deserializer<'de>,
                            {
                                Ok(Self::new(#name::<T, N>::deserialize(deserializer)?))
                            }
                        }
                    }
                }
            }
        )
    } else {

        std::array::from_fn(|idx|
            {
                if idx <= 1
                {
                    let current_name = names[idx];

                    quote! {
                        impl<T> #crate_ident::array::ArrayWithType<T, #dim> for #current_name<T>
                        {
                            type WithType<T2> = #current_name<T2>;
                        }
                        impl<T, Idx> ::std::ops::Index<Idx> for #current_name<T> where [T;#dim] : ::std::ops::Index<Idx>
                        {
                            type Output=<[T;#dim] as ::std::ops::Index<Idx>>::Output;
                            #[inline(always)]
                            fn index(&self, index: Idx) -> &Self::Output { self.array().index(index) }
                        }
                        impl<T, Idx> ::std::ops::IndexMut<Idx> for #current_name<T> where [T;#dim] : ::std::ops::IndexMut<Idx>
                        {
                            #[inline(always)]
                            fn index_mut(&mut self, index: Idx) -> &mut Self::Output { self.array_mut().index_mut(index) }
                        }

                        impl<T, Idx> #crate_ident::hexga_core::collections::Get<Idx> for #current_name<T> where [T;#dim] : #crate_ident::hexga_core::collections::Get<Idx>
                        {
                            type Output = <[T;#dim] as #crate_ident::hexga_core::collections::Get<Idx>>::Output;

                            #[inline(always)]
                            fn get(&self, index: Idx) -> Option<&Self::Output> { #crate_ident::hexga_core::collections::Get::get(self.array(), index) }
                            #[inline(always)]
                            #[track_caller]
                            unsafe fn get_unchecked(&self, index: Idx) -> &Self::Output { unsafe { #crate_ident::hexga_core::collections::Get::get_unchecked(self.array(), index) } }
                        }
                        impl<T, Idx> #crate_ident::hexga_core::collections::TryGet<Idx> for #current_name<T> where [T;#dim] : #crate_ident::hexga_core::collections::TryGet<Idx>
                        {
                            type Error = <[T;#dim] as #crate_ident::hexga_core::collections::TryGet<Idx>>::Error;

                            #[inline(always)]
                            fn try_get(&self, index: Idx) -> Result<&Self::Output, Self::Error> { #crate_ident::hexga_core::collections::TryGet::try_get(self.array(), index) }
                        }
                        impl<T, Idx> #crate_ident::hexga_core::collections::GetMut<Idx> for #current_name<T> where [T;#dim] : #crate_ident::hexga_core::collections::GetMut<Idx>
                        {
                            #[inline(always)]
                            fn get_mut(&mut self, index: Idx) -> Option<&mut Self::Output> { #crate_ident::hexga_core::collections::GetMut::get_mut(self.array_mut(), index) }
                            #[inline(always)]
                            #[track_caller]
                            unsafe fn get_unchecked_mut(&mut self, index: Idx) -> &mut Self::Output { unsafe { #crate_ident::hexga_core::collections::GetMut::get_unchecked_mut(self.array_mut(), index) } }
                        }
                        impl<T, Idx> #crate_ident::hexga_core::collections::TryGetMut<Idx> for #current_name<T> where [T;#dim] : #crate_ident::hexga_core::collections::TryGetMut<Idx>
                        {
                            #[inline(always)]
                            fn try_get_mut(&mut self, index: Idx) -> Result<&mut Self::Output, Self::Error> { #crate_ident::hexga_core::collections::TryGetMut::try_get_mut(self.array_mut(), index) }
                        }


                        impl<T, Idx> #crate_ident::hexga_core::collections::GetManyMut<Idx> for #current_name<T> where [T;#dim] : #crate_ident::hexga_core::collections::GetManyMut<Idx>
                        {
                            #[inline(always)]
                            fn try_get_many_mut<const N: usize>(&mut self, indices: [Idx; N]) -> Result<[&mut Self::Output;N], ManyMutError> { #crate_ident::hexga_core::collections::GetManyMut::try_get_many_mut(self.array_mut(), indices) }
                            #[inline(always)]
                            fn get_many_mut<const N: usize>(&mut self, indices: [Idx; N]) -> Option<[&mut Self::Output;N]> { #crate_ident::hexga_core::collections::GetManyMut::get_many_mut(self.array_mut(), indices) }
                            #[inline(always)]
                            #[track_caller]
                            unsafe fn get_many_unchecked_mut<const N: usize>(&mut self, indices: [Idx; N]) -> [&mut Self::Output;N] { unsafe { #crate_ident::hexga_core::collections::GetManyMut::get_many_unchecked_mut(self.array_mut(), indices) } }
                        }


                        impl<T> ::std::iter::IntoIterator for #current_name<T> where [T;#dim] : ::std::iter::IntoIterator
                        {
                            type Item = <[T;#dim] as ::std::iter::IntoIterator>::Item;
                            type IntoIter = <[T;#dim] as ::std::iter::IntoIterator>::IntoIter;

                            fn into_iter(self) -> Self::IntoIter
                            {
                                <[T;#dim]>::from(self).into_iter()
                            }
                        }

                        impl<'a, T> ::std::iter::IntoIterator for &'a #current_name<T> where &'a [T;#dim] : ::std::iter::IntoIterator
                        {
                            type Item = <&'a [T;#dim] as ::std::iter::IntoIterator>::Item;
                            type IntoIter = <&'a [T;#dim] as ::std::iter::IntoIterator>::IntoIter;

                            fn into_iter(self) -> Self::IntoIter {
                                let array : &[T;#dim] = self.as_ref();
                                array.into_iter()
                            }
                        }

                        impl<'a, T> ::std::iter::IntoIterator for &'a mut #current_name<T> where &'a mut [T;#dim] : ::std::iter::IntoIterator
                        {
                            type Item = <&'a mut [T;#dim] as IntoIterator>::Item;
                            type IntoIter = <&'a mut [T;#dim] as IntoIterator>::IntoIter;

                            fn into_iter(self) -> Self::IntoIter {
                                let array : &mut [T;#dim] = self.as_mut();
                                array.into_iter()
                            }
                        }

                        impl<T> #crate_ident::map::MapIntern for #current_name<T>
                        {
                            type Item=T;
                            fn map_intern<F>(self, f: F) -> Self where F: FnMut(Self::Item) -> Self::Item
                            {
                                Self::from_array(<[T;#dim]>::from(self).map_intern(f))
                            }
                        }
                        impl<T> #crate_ident::map::MapInternWith for #current_name<T>
                        {
                            fn map_with_intern<F>(self, other: Self, f: F) -> Self where F: FnMut(Self::Item, Self::Item) -> Self::Item
                            {
                                Self::from_array(<[T;#dim]>::from(self).map_with_intern(other.into(), f))
                            }
                        }
                        impl<T> #crate_ident::map::Map for #current_name<T>
                        {
                            type WithType<T2> = #current_name<T2>;
                            fn map<T2,F>(self, f: F) -> Self::WithType<T2> where F: FnMut(Self::Item) -> T2 {
                                Self::WithType::from_array(<[T;#dim] as Map>::map(self.into(), f))
                            }
                        }
                        impl<T> #crate_ident::map::MapWith for #current_name<T>
                        {
                            fn map_with<R,Item2,F>(self, other: Self::WithType<Item2>, f: F) -> Self::WithType<R> where F: FnMut(Self::Item, Item2) -> R
                            {
                                Self::WithType::from_array(<[T;#dim]>::from(self).map_with(other.into(), f))
                            }
                        }

                        #crate_ident::map_on::map_on_operator_binary!(
                            (($trait_name: tt, $fn_name: tt)) =>
                            {
                                impl<T> ::std::ops::$trait_name<Self> for #current_name<T>
                                    where T: $trait_name<T>
                                {
                                    type Output=#current_name<<T as ::std::ops::$trait_name<T>>::Output>;
                                    fn $fn_name(self, rhs: Self) -> Self::Output { self.map_with(rhs, T::$fn_name) }
                                }

                                impl<T> ::std::ops::$trait_name<T> for #current_name<T> where T: $trait_name<T> + Copy
                                {
                                    type Output=#current_name<<T as ::std::ops::$trait_name<T>>::Output>;
                                    fn $fn_name(self, rhs: T) -> Self::Output { <[T;#dim]>::from(self).map(|v| v.$fn_name(rhs)).into() }
                                }
                            }
                        );

                        #crate_ident::map_on::map_on_operator_assign!(
                            (($trait_name: tt, $fn_name: tt)) =>
                            {
                                impl<T> ::std::ops::$trait_name<Self> for #current_name<T> where T: $trait_name
                                {
                                    fn $fn_name(&mut self, rhs: Self)
                                    {
                                        let arr : [T; #dim] = rhs.into();
                                        self.array_mut().iter_mut().zip(arr.into_iter()).for_each(|(a, b)| a.$fn_name(b));
                                    }
                                }

                                impl<T> ::std::ops::$trait_name<&Self> for #current_name<T> where T: $trait_name + Copy
                                {
                                    fn $fn_name(&mut self, rhs: &Self) { $trait_name::$fn_name(self,*rhs) }
                                }
                            }
                        );

                        // ================= Unary =========

                        impl<T> ::std::ops::Not for #current_name<T> where T: ::std::ops::Not
                        {
                            type Output = #current_name<T::Output>;
                            fn not(self) -> Self::Output { self.map(::std::ops::Not::not) }
                        }

                        impl<T> ::std::ops::Neg for #current_name<T> where T: ::std::ops::Neg
                        {
                            type Output = #current_name<T::Output>;
                            fn neg(self) -> Self::Output { self.map(::std::ops::Neg::neg) }
                        }

                        // ================= Iter =========

                        impl<T> ::std::iter::Sum for #current_name<T> where Self : #crate_ident::number::Zero + ::std::ops::Add<Self,Output = Self>
                        {
                            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                                iter.fold(<Self as #crate_ident::number::Zero>::ZERO, Self::add)
                            }
                        }

                        impl<T> ::std::iter::Product for #current_name<T> where Self : #crate_ident::number::One + ::std::ops:: Mul<Self,Output = Self>
                        {
                            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                                iter.fold(<Self as #crate_ident::number::One>::ONE, Self::mul)
                            }
                        }

                        unsafe impl<T> #crate_ident::hexga_core::bit::BitAllUsed for #current_name<T> where T:BitAllUsed {}
                        unsafe impl<T> #crate_ident::hexga_core::bit::BitZero for #current_name<T> where T: BitZero {}
                        impl<T> #crate_ident::hexga_core::bit::BitPattern for #current_name<T> where T: BitPattern, [T::Bits;#dim] : BitAnyPattern
                        {
                            type Bits = [T::Bits;#dim];

                            fn is_bit_pattern_valid(bits: &Self::Bits) -> bool {
                                bits.iter().all(|b| T::is_bit_pattern_valid(b))
                            }
                        }

                        #crate_ident::map_on::map_on_constant!
                        (
                            (($trait_name: tt, $constant_name: tt)) =>
                            {
                                impl<T> #crate_ident::number::$trait_name for #current_name<T> where T: #crate_ident::number::$trait_name + ::std::marker::Copy
                                {
                                    const $constant_name: Self = Self::from_array(<[T;#dim]>::$constant_name);
                                }
                            }
                        );

                        #crate_ident::map_on::map_on_std_fmt!(
                            ($trait_name :ident) =>
                            {
                                impl<T> std::fmt::$trait_name for #current_name<T> where T: std::fmt::$trait_name
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
                        );
                    }
                }else
                {
                    if !has_non_exhaustive
                    {
                        // serialize like a tuple
                        quote!
                        {
                            #[cfg(feature = "serde")]
                            impl<T> ::serde::Serialize for #name<T>
                            where
                                T: ::serde::Serialize,
                            {
                                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                where
                                    S: ::serde::Serializer,
                                {
                                    use ::serde::ser::SerializeTuple;

                                    let arr = self.as_array();
                                    let mut tup = serializer.serialize_tuple(#dim)?;
                                    for item in arr {
                                        tup.serialize_element(item)?;
                                    }
                                    tup.end()
                                }
                            }

                            #[cfg(feature = "serde")]
                            const _ : () =
                            {
                                // Thank serde for not supporting the deserialization of variadic array...
                                #[derive(Debug)]
                                struct Arr<T>(pub [T; #dim]);

                                impl<'de, T> ::serde::Deserialize<'de> for #name<T> where T: ::serde::Deserialize<'de>
                                {
                                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                                    where
                                        D: ::serde::Deserializer<'de>,
                                    {
                                        struct ArrVisitor<T>(::std::marker::PhantomData<T>);

                                        impl<'de, T> ::serde::de::Visitor<'de> for ArrVisitor<T>
                                        where
                                            T: ::serde::Deserialize<'de>,
                                        {
                                            type Value = Arr<T>;

                                            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                                write!(formatter, "an {}", std::any::type_name::<#name<T>>())
                                            }

                                            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                                            where
                                                A: ::serde::de::SeqAccess<'de>,
                                            {
                                                // SAFETY: We'll ensure every element is initialized or properly dropped
                                                let mut data: [::std::mem::MaybeUninit<T>; #dim] = unsafe { ::std::mem::MaybeUninit::uninit().assume_init() };
                                                for i in 0..#dim {
                                                    match seq.next_element()? {
                                                        Some(value) => data[i] = ::std::mem::MaybeUninit::new(value),
                                                        None => {
                                                            if ::core::mem::needs_drop::<T>() {
                                                                // Drop any already initialized elements before returning
                                                                for j in 0..i {
                                                                    unsafe { ::std::ptr::drop_in_place(data[j].as_mut_ptr()) };
                                                                }
                                                            }
                                                            return Err(::serde::de::Error::invalid_length(i, &self));
                                                        }
                                                    }
                                                }

                                                // Ensure no extra elements
                                                if seq.next_element::<::serde::de::IgnoredAny>()?.is_some() {
                                                    if ::core::mem::needs_drop::<T>() {
                                                        for i in 0..#dim {
                                                            unsafe { ::core::ptr::drop_in_place(data[i].as_mut_ptr()) };
                                                        }
                                                    }
                                                    return Err(::serde::de::Error::invalid_length(#dim + 1, &self));
                                                }

                                                // SAFETY: All elements are initialized
                                                let result = unsafe { ::std::ptr::read(&data as *const _ as *const [T; #dim]) };
                                                Ok(Arr(result))
                                            }
                                        }

                                        deserializer.deserialize_tuple(#dim, ArrVisitor::<T>(::std::marker::PhantomData)).map(|arr| #name::<T>::from(arr.0))
                                    }
                                }
                            };

                            #[cfg(feature = "serde")]
                            impl<T> ::serde::Serialize for #name_coef<T>
                                where #name<T>: ::serde::Serialize,
                            {
                                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                where
                                    S: ::serde::Serializer,
                                {
                                    self.value.serialize(serializer)
                                }
                            }

                            #[cfg(feature = "serde")]
                            impl<'de, T> ::serde::Deserialize<'de> for #name_coef<T>
                                where #name<T>: ::serde::Deserialize<'de>
                            {
                                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                                where
                                    D: ::serde::Deserializer<'de>,
                                {
                                    Ok(Self::new(#name::<T>::deserialize(deserializer)?))
                                }
                            }
                        }
                    }else
                    {
                        let fields = match &input.fields {
                            syn::Fields::Named(f) => &f.named,
                            _ => {
                                return syn::Error::new_spanned(
                                    &input,
                                    "math_vec only supports structs with named fields",
                                )
                                .to_compile_error()
                                .into();
                            }
                        };

                        // serialize field by field
                        let field_idents: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
                        //let field_names: Vec<_> = field_idents.iter().map(|f| f.to_string()).collect();
                        let num_fields = field_idents.len();

                        let name_str = name.to_string();
                        let name_coef_str = name_coef.to_string();

                        quote! {
                            #[cfg(feature = "serde")]
                            impl<'de, T> ::serde::Serialize for #name<T>
                            where
                                T: ::serde::Serialize
                            {
                                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                where
                                    S: ::serde::Serializer
                                {
                                    let mut state = serializer.serialize_struct(stringify!(#name), #num_fields)?;
                                    #(state.serialize_field(stringify!(#field_idents), &self.#field_idents)?;)*
                                    state.end()
                                }
                            }

                            #[cfg(feature = "serde")]
                            impl<'de, T> ::serde::Deserialize<'de> for #name<T>
                            where
                                T: ::serde::Deserialize<'de> + #crate_ident::number::Zero,
                            {
                                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                                where
                                    D: ::serde::Deserializer<'de>,
                                {
                                    fn __zero<T: #crate_ident::number::Zero>() -> T {
                                        T::ZERO
                                    }

                                    #[derive(::serde::Deserialize)]
                                    #[allow(non_camel_case_types)]
                                    #[serde(rename = #name_str)]

                                    struct __Temp<T>
                                    where
                                        T: #crate_ident::number::Zero,
                                    {
                                        #( #[serde(default = "__zero")] #field_idents: T, )*
                                    }

                                    // Use super::#name to construct the original type
                                    let temp = __Temp::<T>::deserialize(deserializer)?;
                                    Ok(#name {
                                        #(#field_idents: temp.#field_idents),*
                                    })
                                }
                            }



                            #[cfg(feature = "serde")]
                            impl<'de, T> ::serde::Serialize for #name_coef<T>
                            where
                                T: ::serde::Serialize
                            {
                                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                where
                                    S: ::serde::Serializer
                                {
                                    let mut state = serializer.serialize_struct(stringify!(#name_coef), #num_fields)?;
                                    #(state.serialize_field(stringify!(#field_idents), &self.value.#field_idents)?;)*
                                    state.end()
                                }
                            }

                            #[cfg(feature = "serde")]
                            impl<'de, T> ::serde::Deserialize<'de> for #name_coef<T>
                            where
                                T: ::serde::Deserialize<'de> + #crate_ident::number::One,
                            {
                                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                                where
                                    D: ::serde::Deserializer<'de>,
                                {
                                    fn __one<T: #crate_ident::number::One>() -> T {
                                        T::ONE
                                    }

                                    #[derive(::serde::Deserialize)]
                                    #[allow(non_camel_case_types)]
                                    #[serde(rename = #name_coef_str)]
                                    struct __Temp<T>
                                    where
                                        T: #crate_ident::number::One,
                                    {
                                        #( #[serde(default = "__one")] #field_idents: T, )*
                                    }

                                    // Use super::#name to construct the original type
                                    let temp = __Temp::<T>::deserialize(deserializer)?;
                                    Ok(#name_coef::new(#name{
                                        #(#field_idents: temp.#field_idents),*
                                    }))
                                }
                            }

                        }
                    }
                }
            }
        )
    };

    fn where_with_bounds(
        base: Option<&syn::WhereClause>,
        generics: &syn::Generics,
        bound: syn::Path,
    ) -> syn::WhereClause {
        let mut wc = base.cloned().unwrap_or_else(|| syn::WhereClause {
            where_token: Default::default(),
            predicates: Default::default(),
        });

        wc.predicates.extend::<Vec<syn::WherePredicate>>(
            generics.type_params().map(|p| {
                let ident = &p.ident;
                syn::parse_quote!(#ident: #bound)
            }).collect()
        );

        wc
    }

    let clone_where_clause = where_with_bounds(where_clause, &input.generics, syn::parse_quote!(::std::clone::Clone));
    let copy_where_clause = where_with_bounds(where_clause, &input.generics, syn::parse_quote!(::std::marker::Copy));
    let partial_eq_where_clause = where_with_bounds(where_clause, &input.generics, syn::parse_quote!(::std::cmp::PartialEq));
    let eq_where_clause = where_with_bounds(where_clause, &input.generics, syn::parse_quote!(::std::cmp::Eq));
    let partial_ord_where_clause = where_with_bounds(where_clause, &input.generics, syn::parse_quote!(::std::cmp::PartialOrd));
    let ord_where_clause = where_with_bounds(where_clause, &input.generics, syn::parse_quote!(::std::cmp::Ord));
    let hash_where_clause = where_with_bounds(where_clause, &input.generics, syn::parse_quote!(::std::hash::Hash));

    let [basic_impl, coef_impl] = std::array::from_fn(|idx|
        {
            let current_name = names[idx];
            quote! {


                impl #impl_generics #current_name #ty_generics {
                    pub const LEN: usize = #dim;

                    #[doc(hidden)]
                    #[allow(dead_code)]
                    pub(crate) const IS_VALID: () =
                    {
                        assert!(std::mem::size_of::<Self>() == std::mem::size_of::<[T;#dim]>());
                    };

                    pub const fn from_array(array : [T;#dim]) -> Self
                    {
                        let _ = Self::IS_VALID;
                        let s = unsafe { std::ptr::read(&array as *const [T;#dim] as *const Self) };
                        std::mem::forget(array);
                        s
                    }

                    pub const fn to_array(self) -> [T;#dim]
                    {
                        let _ = Self::IS_VALID;
                        let s = unsafe { std::ptr::read(&self as *const Self as *const [T;#dim]) };
                        std::mem::forget(self);
                        s
                    }

                    #[inline(always)]
                    pub const fn array(&self) -> &[T; #dim] { let _ = Self::IS_VALID; unsafe { std::mem::transmute(self) } }
                    #[inline(always)]
                    pub const fn array_mut(&mut self) -> &mut[T; #dim] { let _ = Self::IS_VALID; unsafe { std::mem::transmute(self) } }

                    #crate_ident::impl_number_basic_trait!();
                }

                impl #impl_generics ::std::convert::From<[T; #dim]> for #current_name #ty_generics { fn from(value: [T; #dim]) -> Self { Self::from_array(value) } }
                impl #impl_generics ::std::convert::From<#current_name #ty_generics> for [T; #dim] { fn from(value: #current_name #ty_generics) -> Self { value.to_array() } }

                impl #impl_generics ::std::convert::AsRef<[T; #dim]> for #current_name #ty_generics { fn as_ref(&self) -> &[T; #dim] { self.array() } }
                impl #impl_generics ::std::convert::AsMut<[T; #dim]> for #current_name #ty_generics { fn as_mut(&mut self) -> &mut [T; #dim] { self.array_mut() } }


                impl #impl_generics ::std::clone::Clone for #current_name #ty_generics #clone_where_clause
                {
                    fn clone(&self) -> Self { self.array().clone().into() }
                }
                impl #impl_generics ::std::marker::Copy for #current_name #ty_generics #copy_where_clause {}

                impl #impl_generics ::std::cmp::PartialEq for #current_name #ty_generics #partial_eq_where_clause
                {
                    fn eq(&self, rhs : &Self) -> bool { self.array() == rhs.array() }
                }
                impl #impl_generics ::std::cmp::Eq for #current_name #ty_generics #eq_where_clause {}

                impl #impl_generics ::std::cmp::PartialOrd for #current_name #ty_generics #partial_ord_where_clause
                {
                    fn partial_cmp(&self, rhs : &Self) -> ::std::option::Option<::std::cmp::Ordering> { ::std::cmp::PartialOrd::partial_cmp(self.array(), rhs.array()) }
                }
                impl #impl_generics ::std::cmp::Ord for #current_name #ty_generics #ord_where_clause
                {
                    fn cmp(&self, rhs : &Self) -> ::std::cmp::Ordering { ::std::cmp::Ord::cmp(self.array(), rhs.array()) }
                }

                impl #impl_generics ::std::hash::Hash for #current_name #ty_generics #hash_where_clause
                {
                    fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher { self.as_ref().hash(state); }
                }

                impl #impl_generics #crate_ident::array::Array<T, #dim> for #current_name #ty_generics
                {
                    #[inline(always)]
                    fn array(&self) -> &[T; #dim] { self.array() }
                    #[inline(always)]
                    fn array_mut(&mut self) -> &mut[T; #dim] { self.array_mut() }
                }
            }
        }
    );


    let expanded = quote!
    {
        #input

        impl #impl_generics ::std::convert::From<#name_coef #ty_generics> for #name #ty_generics
        {
            #[inline(always)]
            fn from(coef: #name_coef #ty_generics) -> Self { coef.value }
        }

        /// Default value is One.
        ///
        /// Same for missing value during deserialization (useful with `#[non_exhaustive]`).
        pub struct #name_coef #impl_generics
        {
            pub value: #name #ty_generics ,
        }

        impl #impl_generics ::std::convert::From<#name #ty_generics> for #name_coef #ty_generics
        {
            #[inline(always)]
            fn from(value: #name #ty_generics) -> Self { Self::new(value) }
        }
        impl #impl_generics #name_coef #ty_generics
        {
            #[inline(always)]
            pub const fn new(value: #name #ty_generics) -> Self { Self { value } }
        }

        /*
        Each type define what is the Default;
        Zero for Vector
        Unit Rectangle for Rectangle,
        White for Color

        impl #impl_generics ::std::default::Default for #name #ty_generics
            where Self: #crate_ident::number::Zero
        {
            fn default() -> Self { <Self as #crate_ident::number::Zero>::ZERO }
        }
        */

        impl #impl_generics ::std::default::Default for #name_coef #ty_generics
            where Self: #crate_ident::number::One
        {
            fn default() -> Self { <Self as #crate_ident::number::One>::ONE }
        }

        #basic_impl
        #coef_impl

        #array_impl
        #array_impl_coef
        #specific_impl
    };


    expanded.into()
}

// Todo: add some method to iter with the field name,
// or some method to only iter in non default field (non zero or non one for coef) : iter() -> impl Iterator<Item=(&'static str,T)>