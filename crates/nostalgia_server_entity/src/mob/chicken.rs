use crate::{id, Entity, EntityFlags};
use macros::entity;
use types::Vector3;

#[entity(id = id::MOB_CHICKEN)]
#[derive(Debug, Default)]
pub struct Chicken {}

impl Chicken {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

mod handler {
    use super::*;

    pub fn spawn(chicken: &mut Chicken) {
        println!("Chicken {} spawned", chicken.id);
    }

    pub fn despawn(chicken: &mut Chicken) {
        println!("Chicken {} despawned", chicken.id);
    }

    pub fn update(chicken: &mut Chicken) {
        println!("Chicken {} updated", chicken.id);
    }
}
