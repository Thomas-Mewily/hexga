use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Item, DeriveInput};

#[proc_macro_derive(Save)]
pub fn derive_save(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;

    let quoted = quote! {
        impl ::hexga_io::IoLoad for #ident
        where
            #ident: ::serde::ser::Serialize
        {}
    };

    quoted.into()
}

#[proc_macro_derive(Load)]
pub fn derive_load(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;

    let quoted = quote! {
        impl ::hexga_io::IoSave for #ident
        where
            #ident: for<'de> ::serde::de::Deserialize<'de>
        {}
    };

    quoted.into()
}


#[proc_macro_attribute]
pub fn io(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    match item {
        Item::Struct(ref s) => expand_io_struct(s),
        _ => {
            syn::Error::new_spanned(item, "#[Io] can only be used on structs")
                .to_compile_error()
                .into()
        }
    }
}

fn expand_io_struct(struct_item: &ItemStruct) -> TokenStream {
    let ident = &struct_item.ident;

    let expanded = quote! {
        #[derive(::serde::Serialize, ::serde::Deserialize)]
        #struct_item

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