use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput};

pub fn urid_collection_derive_impl(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let struct_name = input.ident;

    let fields = match input.data {
        Data::Struct(DataStruct { fields, .. }) => fields,
        _ => panic!("Only structs can implement `URIDCollection`"),
    };

    let field_inits = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .map(|ident| quote! {#ident: map.populate_collection()?,});

    let implementation = quote! {
        impl URIDCollection for #struct_name {
            fn from_map<M: Map + ?Sized>(map: &M) -> Option<Self> {
                Some(Self {
                    #(#field_inits)*
                })
            }
        }
    };

    implementation.into()
}
