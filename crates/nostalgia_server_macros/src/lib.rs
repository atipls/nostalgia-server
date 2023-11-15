use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parser,
    parse_macro_input,
    ExprAssign, Field, Fields, ItemStruct,
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
        quote! { health: u8 },
        quote! { air: u16 },
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
        impl crate::Entity for #item_struct_name {
            fn entity_id() -> u8 { #entity_id }
            fn id(&self) -> i32 { self.id }

            fn flags(&self) -> EntityFlags { self.flags }
            fn flags_mut(&mut self) -> &mut EntityFlags { &mut self.flags }

            fn position(&self) -> Vector3 { self.position }
            fn position_mut(&mut self) -> &mut Vector3 { &mut self.position }
            
            fn velocity(&self) -> Vector3 { self.velocity }
            fn velocity_mut(&mut self) -> &mut Vector3 { &mut self.velocity }

            fn on_ground(&self) -> bool { self.on_ground }
            fn on_ground_mut(&mut self) -> &mut bool { &mut self.on_ground }

            fn pitch(&self) -> u8 { self.pitch }
            fn pitch_mut(&mut self) -> &mut u8 { &mut self.pitch }

            fn yaw(&self) -> u8 { self.yaw }
            fn yaw_mut(&mut self) -> &mut u8 { &mut self.yaw }

            fn spawn(&mut self) { handler::spawn(self); }
            fn despawn(&mut self) { handler::despawn(self); }
            fn update(&mut self) { handler::update(self); }

            fn serialize(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<()> { handler::serialize(self, cursor) }
            fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<Self> where Self: Sized { handler::parse(cursor) }
        }

        impl crate::LivingEntity for #item_struct_name {
            fn health(&self) -> u8 { self.health }
            fn health_mut(&mut self) -> &mut u8 { &mut self.health }

            fn air(&self) -> u16 { self.air }
            fn air_mut(&mut self) -> &mut u16 { &mut self.air }

            fn damage(&mut self) { unimplemented!() }
            fn heal(&mut self) { unimplemented!() }
        }
    };

    quote! {
        #item_struct
        #item_entity_impl
    }
    .into()
}
