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
    fn spawn(&self);
    fn despawn(&self);
    fn update(&self);
}

pub trait LivingEntity: Entity {
    fn damage(&self);
    fn heal(&self);
}
