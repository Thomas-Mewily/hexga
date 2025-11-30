use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, parse_quote};
use quote::quote;

#[proc_macro_derive(Zeroable)]
pub fn derive_zeroable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = input.generics;

    // Split generics for: impl <...> | Type<...>
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Optionally enforce trait bounds for generic parameters:
    let mut where_clause = where_clause.cloned();
    {
        let wc = where_clause.get_or_insert_with(|| syn::WhereClause {
            where_token: Default::default(),
            predicates: Default::default(),
        });

        for param in generics.params.iter() {
            if let syn::GenericParam::Type(ty_param) = param {
                let ident = &ty_param.ident;
                wc.predicates.push(parse_quote! { #ident: ::hexga_mem::BitsZero });
            }
        }
    }

    let expanded = quote! {
        unsafe impl #impl_generics ::hexga_mem::BitsZero for #name #ty_generics
        #where_clause
        {
        }
    };

    TokenStream::from(expanded)
}
