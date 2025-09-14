use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, GenericParam, Generics, Item};

#[proc_macro_derive(Save)]
pub fn derive_save(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;
    let generics = add_save_trait_bounds(ast.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::hexga_io::IoSave for #ident #ty_generics #where_clause {}
    };

    expanded.into()
}

// Adds `T: ::hexga_io::IoLoad` for each generic type parameter `T`
fn add_save_trait_bounds(mut generics: Generics) -> Generics {
    for param in generics.params.iter_mut() {
        if let GenericParam::Type(ty) = param {
            ty.bounds.push(parse_quote!(::hexga_io::IoSave));
        }
    }
    generics
}




#[proc_macro_derive(Load)]
pub fn derive_load(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;
    let generics = add_load_trait_bounds(ast.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::hexga_io::IoLoad for #ident #ty_generics #where_clause {}
    };

    expanded.into()
}

// Adds `T: ::hexga_io::IoLoad` for each generic type parameter `T`
fn add_load_trait_bounds(mut generics: Generics) -> Generics {
    for param in generics.params.iter_mut() {
        if let GenericParam::Type(ty) = param {
            ty.bounds.push(parse_quote!(::hexga_io::IoLoad));
        }
    }
    generics
}



#[proc_macro_attribute]
pub fn io(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    let (ident, item_tokens) = match item {
        Item::Struct(s) => (&s.ident.clone(), quote! { #s }),
        Item::Enum(e) => (&e.ident.clone(), quote! { #e }),
        _ => {
            return syn::Error::new_spanned(item, "#[io] can only be used on structs or enums")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        #[derive(::serde::Serialize, ::serde::Deserialize)]
        #item_tokens

        impl ::hexga_io::IoSave for #ident
        where
            #ident: for<'de> ::serde::de::Deserialize<'de>
        {}

        impl ::hexga_io::IoLoad for #ident
        where
            #ident: ::serde::ser::Serialize
        {}
    };

    expanded.into()
}
