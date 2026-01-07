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


    let mut where_clause = where_clause.cloned();
    let clone_where_clause = where_clause.get_or_insert_with(|| WhereClause {
        where_token: Default::default(),
        predicates: Default::default(),
    });

    for param in input.generics.type_params() {
        let ident = &param.ident;
        clone_where_clause.predicates.push(syn::parse_quote! {
            #ident: ::std::clone::Clone
        });
    }

    let expanded = quote! {
        #input

        impl #impl_generics #name #ty_generics #clone_where_clause {
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

        impl #impl_generics ::std::clone::Clone for #name #ty_generics #where_clause
        {
            fn clone(&self) -> Self { self.array().clone().into() }
        }
    };

    expanded.into()
}
