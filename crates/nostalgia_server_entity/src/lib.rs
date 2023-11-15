use types::Vector3;
use std::io::Cursor;
use std::io::Result;

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
    fn flags_mut(&mut self) -> &mut EntityFlags;

    fn position(&self) -> Vector3;
    fn position_mut(&mut self) -> &mut Vector3;

    fn velocity(&self) -> Vector3;
    fn velocity_mut(&mut self) -> &mut Vector3;

    fn on_ground(&self) -> bool;
    fn on_ground_mut(&mut self) -> &mut bool;

    fn pitch(&self) -> u8;
    fn pitch_mut(&mut self) -> &mut u8;

    fn yaw(&self) -> u8;
    fn yaw_mut(&mut self) -> &mut u8;

    fn spawn(&mut self);
    fn despawn(&mut self);
    fn update(&mut self);

    fn serialize(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<()>;
    fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<Self> where Self: Sized;
}

pub trait LivingEntity: Entity {
    fn health(&self) -> u8;
    fn health_mut(&mut self) -> &mut u8;

    fn air(&self) -> u16;
    fn air_mut(&mut self) -> &mut u16;
    
    fn damage(&mut self);
    fn heal(&mut self);
}
