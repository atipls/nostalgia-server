use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse,
    parse::{Parse, ParseStream, Parser, Result},
    parse_macro_input,
    punctuated::Punctuated,
    ExprAssign, Field, Fields, Ident, ItemStruct, LitStr, Token,
};

#[proc_macro_attribute]
pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let entity_id_argument = parse_macro_input!(args as ExprAssign);

    let entity_fields = [
        quote! { id: i32 },
        quote! { flags: EntityFlags },
        quote! { position: Vector3 },
        quote! { velocity: Vector3 },
        quote! { on_ground: bool },
        quote! { pitch: u8 },
        quote! { yaw: u8 },
    ];

    let Fields::Named(ref mut fields) = item_struct.fields else {
        panic!("Entity must be a struct with named fields");
    };

    fields.named.extend(
        entity_fields
            .iter()
            .map(|field| Field::parse_named.parse2(field.clone()).unwrap()),
    );

    let entity_id = entity_id_argument.right;
    let item_struct_name = &item_struct.ident;
    let item_entity_impl = quote! {
        impl Entity for #item_struct_name {
            fn entity_id() -> u8 { #entity_id }
            fn id(&self) -> i32 { self.id }

            fn flags(&self) -> EntityFlags { self.flags }
            fn position(&self) -> Vector3 { self.position }
            fn velocity(&self) -> Vector3 { self.velocity }
            fn on_ground(&self) -> bool { self.on_ground }

            fn pitch(&self) -> u8 { self.pitch }
            fn yaw(&self) -> u8 { self.yaw }

            fn spawn(&mut self) { handler::spawn(self); }
            fn despawn(&mut self) { handler::despawn(self); }
            fn update(&mut self) { handler::update(self); }
        }
    };

    quote! {
        #item_struct
        #item_entity_impl
    }
    .into()
}
