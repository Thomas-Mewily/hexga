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
            //$crate::impl_number_basic_trait!();
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
    };

    expanded.into()
}
