use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields, GenericParam, Type};
use syn::{WhereClause, WherePredicate};

#[proc_macro_attribute]
pub fn math_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();

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


    let is_array_struct = input.generics.params.iter().any(|p| matches!(p, syn::GenericParam::Const(_)));
    let array_impl = if is_array_struct {
        quote! {
            impl<T, const N: usize> ::hexga_math::array::ArrayWithType<T, N> for #name<T, N>
            {
                type WithType<T2> = #name<T2, N>;
            }
            impl<T, const N : usize> ::hexga_math::array::ArrayWithSize<T, N> for #name<T,N>
            {
                type WithSize<const M:usize>=#name<T,M>;
            }

            impl<T, const N : usize, Idx> ::std::ops::Index<Idx> for #name<T,N> where [T;N] : ::std::ops::Index<Idx>
            {
                type Output=<[T;N] as ::std::ops::Index<Idx>>::Output;
                #[inline(always)]
                fn index(&self, index: Idx) -> &Self::Output { self.array().index(index) }
            }
            impl<T, const N : usize, Idx> ::std::ops::IndexMut<Idx> for #name<T,N> where [T;N] : ::std::ops::IndexMut<Idx>
            {
                #[inline(always)]
                fn index_mut(&mut self, index: Idx) -> &mut Self::Output { self.array_mut().index_mut(index) }
            }


            impl<T, const N : usize, Idx> ::hexga_math::hexga_core::collections::Get<Idx> for #name<T,N> where [T;N] : ::hexga_math::hexga_core::collections::Get<Idx>
            {
                type Output = <[T;N] as ::hexga_math::hexga_core::collections::Get<Idx>>::Output;

                #[inline(always)]
                fn get(&self, index: Idx) -> Option<&Self::Output> { ::hexga_math::hexga_core::collections::Get::get(self.array(), index) }
                #[inline(always)]
                #[track_caller]
                unsafe fn get_unchecked(&self, index: Idx) -> &Self::Output { unsafe { ::hexga_math::hexga_core::collections::Get::get_unchecked(self.array(), index) } }
            }
            impl<T, const N : usize, Idx> ::hexga_math::hexga_core::collections::TryGet<Idx> for #name<T,N> where [T;N] : ::hexga_math::hexga_core::collections::TryGet<Idx>
            {
                type Error = <[T;N] as ::hexga_math::hexga_core::collections::TryGet<Idx>>::Error;

                #[inline(always)]
                fn try_get(&self, index: Idx) -> Result<&Self::Output, Self::Error>
                {
                    ::hexga_math::hexga_core::collections::TryGet::try_get(self.array(), index)
                }
            }
            impl<T, const N : usize, Idx> ::hexga_math::hexga_core::collections::GetMut<Idx> for #name<T,N> where [T;N] : ::hexga_math::hexga_core::collections::GetMut<Idx>
            {
                #[inline(always)]
                fn get_mut(&mut self, index: Idx) -> Option<&mut Self::Output> { ::hexga_math::hexga_core::collections::GetMut::get_mut(self.array_mut(), index) }
                #[inline(always)]
                #[track_caller]
                unsafe fn get_unchecked_mut(&mut self, index: Idx) -> &mut Self::Output { unsafe { ::hexga_math::hexga_core::collections::GetMut::get_unchecked_mut(self.array_mut(), index) } }
            }
            impl<T, const N : usize, Idx> ::hexga_math::hexga_core::collections::TryGetMut<Idx> for #name<T,N> where [T;N] : ::hexga_math::hexga_core::collections::TryGetMut<Idx>
            {
                #[inline(always)]
                fn try_get_mut(&mut self, index: Idx) -> Result<&mut Self::Output, Self::Error> { ::hexga_math::hexga_core::collections::TryGetMut::try_get_mut(self.array_mut(), index) }
            }


            impl<T, const N : usize, Idx> ::hexga_math::hexga_core::collections::GetManyMut<Idx> for #name<T,N> where [T;N] : ::hexga_math::hexga_core::collections::GetManyMut<Idx>
            {
                #[inline(always)]
                fn try_get_many_mut<const N2: usize>(&mut self, indices: [Idx; N2]) -> Result<[&mut Self::Output;N2], ManyMutError> { ::hexga_math::hexga_core::collections::GetManyMut::try_get_many_mut(self.array_mut(), indices) }
                #[inline(always)]
                fn get_many_mut<const N2: usize>(&mut self, indices: [Idx; N2]) -> Option<[&mut Self::Output;N2]> { ::hexga_math::hexga_core::collections::GetManyMut::get_many_mut(self.array_mut(), indices) }

                #[inline(always)]
                #[track_caller]
                unsafe fn get_many_unchecked_mut<const N2: usize>(&mut self, indices: [Idx; N2]) -> [&mut Self::Output;N2] { unsafe { ::hexga_math::hexga_core::collections::GetManyMut::get_many_unchecked_mut(self.array_mut(), indices) } }
            }


            impl<T, const N : usize> ::std::iter::IntoIterator for #name<T,N> where [T;N] : ::std::iter::IntoIterator
            {
                type Item = <[T;N] as ::std::iter::IntoIterator>::Item;
                type IntoIter = <[T;N] as ::std::iter::IntoIterator>::IntoIter;

                fn into_iter(self) -> Self::IntoIter
                {
                    <[T;N]>::from(self).into_iter()
                }
            }

            impl<'a, T, const N : usize> ::std::iter::IntoIterator for &'a #name<T,N> where &'a [T;N] : ::std::iter::IntoIterator
            {
                type Item = <&'a [T;N] as ::std::iter::IntoIterator>::Item;
                type IntoIter = <&'a [T;N] as ::std::iter::IntoIterator>::IntoIter;

                fn into_iter(self) -> Self::IntoIter
                {
                    let array : &[T;N] = self.as_ref();
                    array.into_iter()
                }
            }

            impl<'a, T, const N : usize> ::std::iter::IntoIterator for &'a mut #name<T,N> where &'a mut [T;N] : ::std::iter::IntoIterator
            {
                type Item = <&'a mut [T;N] as IntoIterator>::Item;
                type IntoIter = <&'a mut [T;N] as IntoIterator>::IntoIter;

                fn into_iter(self) -> Self::IntoIter {
                    let array : &mut [T;N] = self.as_mut();
                    array.into_iter()
                }
            }


            impl<T, const N : usize> ::hexga_math::map::MapIntern for #name<T,N>
            {
                type Item=T;
                fn map_intern<F>(self, f: F) -> Self where F: FnMut(Self::Item) -> Self::Item
                {
                    Self::from_array(<[T;N]>::from(self).map_intern(f))
                }
            }
            impl<T, const N : usize> ::hexga_math::map::MapInternWith for #name<T,N>
            {
                fn map_with_intern<F>(self, other: Self, f: F) -> Self where F: FnMut(Self::Item, Self::Item) -> Self::Item
                {
                    Self::from_array(<[T;N]>::from(self).map_with_intern(other.into(), f))
                }
            }
            impl<T, const N : usize> ::hexga_math::map::Map for #name<T,N>
            {
                type WithType<T2> = #name<T2,N>;
                fn map<T2,F>(self, f: F) -> Self::WithType<T2> where F: FnMut(Self::Item) -> T2 {
                    Self::WithType::from_array(<[T;N] as Map>::map(self.into(), f))
                }
            }
            impl<T, const N : usize> ::hexga_math::map::MapWith for #name<T,N>
            {
                fn map_with<R,Item2,F>(self, other: Self::WithType<Item2>, f: F) -> Self::WithType<R> where F: FnMut(Self::Item, Item2) -> R
                {
                    Self::WithType::from_array(<[T;N]>::from(self).map_with(other.into(), f))
                }
            }
        }
    } else {
        quote! {
            impl<T> ::hexga_math::array::ArrayWithType<T, #dim> for #name<T>
            {
                type WithType<T2> = #name<T2>;
            }
            impl<T, Idx> ::std::ops::Index<Idx> for #name<T> where [T;#dim] : ::std::ops::Index<Idx>
            {
                type Output=<[T;#dim] as ::std::ops::Index<Idx>>::Output;
                #[inline(always)]
                fn index(&self, index: Idx) -> &Self::Output { self.array().index(index) }
            }
            impl<T, Idx> ::std::ops::IndexMut<Idx> for #name<T> where [T;#dim] : ::std::ops::IndexMut<Idx>
            {
                #[inline(always)]
                fn index_mut(&mut self, index: Idx) -> &mut Self::Output { self.array_mut().index_mut(index) }
            }
        }
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



    let expanded = quote! {
        #input

        impl #impl_generics #name #ty_generics {
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

            // Todo:
            //#crate::impl_number_basic_trait!();
        }

        impl #impl_generics ::std::convert::From<[T; #dim]> for #name #ty_generics { fn from(value: [T; #dim]) -> Self { Self::from_array(value) } }
        impl #impl_generics ::std::convert::From<#name #ty_generics> for [T; #dim] { fn from(value: #name #ty_generics) -> Self { value.to_array() } }

        impl #impl_generics ::std::convert::AsRef<[T; #dim]> for #name #ty_generics { fn as_ref(&self) -> &[T; #dim] { self.array() } }
        impl #impl_generics ::std::convert::AsMut<[T; #dim]> for #name #ty_generics { fn as_mut(&mut self) -> &mut [T; #dim] { self.array_mut() } }


        impl #impl_generics ::std::clone::Clone for #name #ty_generics #clone_where_clause
        {
            fn clone(&self) -> Self { self.array().clone().into() }
        }
        impl #impl_generics ::std::marker::Copy for #name #ty_generics #copy_where_clause {}

        impl #impl_generics ::std::cmp::PartialEq for #name #ty_generics #partial_eq_where_clause
        {
            fn eq(&self, rhs : &Self) -> bool { self.array() == rhs.array() }
        }
        impl #impl_generics ::std::cmp::Eq for #name #ty_generics #eq_where_clause {}

        impl #impl_generics ::std::cmp::PartialOrd for #name #ty_generics #partial_ord_where_clause
        {
            fn partial_cmp(&self, rhs : &Self) -> ::std::option::Option<::std::cmp::Ordering> { ::std::cmp::PartialOrd::partial_cmp(self.array(), rhs.array()) }
        }
        impl #impl_generics ::std::cmp::Ord for #name #ty_generics #ord_where_clause
        {
            fn cmp(&self, rhs : &Self) -> ::std::cmp::Ordering { ::std::cmp::Ord::cmp(self.array(), rhs.array()) }
        }

        impl #impl_generics ::std::hash::Hash for #name #ty_generics #hash_where_clause
        {
            fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher { self.as_ref().hash(state); }
        }

        impl #impl_generics ::hexga_math::array::Array<T, #dim> for #name #ty_generics
        {
            #[inline(always)]
            fn array(&self) -> &[T; #dim] { self.array() }
            #[inline(always)]
            fn array_mut(&mut self) -> &mut[T; #dim] { self.array_mut() }
        }

        #array_impl
    };

    expanded.into()
}
