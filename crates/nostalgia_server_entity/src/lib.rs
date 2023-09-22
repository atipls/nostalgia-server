use types::Vector3;

pub mod id;
pub mod mob;

pub mod entity_flags {
    pub const ON_FIRE: u8 = 0x01;
    pub const UNKNOWN1: u8 = 0x02;
    pub const IS_CROUCHED: u8 = 0x04;
    pub const IS_IN_ACTION: u8 = 0x10;
}

pub type EntityFlags = u8;

pub trait Entity {
    fn entity_id() -> u8;
    fn id(&self) -> i32;

    fn flags(&self) -> EntityFlags;
    fn position(&self) -> Vector3;
    fn velocity(&self) -> Vector3;
    fn on_ground(&self) -> bool;

    fn pitch(&self) -> u8;
    fn yaw(&self) -> u8;

    fn spawn(&mut self);
    fn despawn(&mut self);
    fn update(&mut self);
}

pub trait LivingEntity: Entity {
    fn damage(&self);
    fn heal(&self);
}
