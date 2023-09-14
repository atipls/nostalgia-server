use crate::EntityFlags;
use types::Vector3;

pub struct Chicken {
    id: i32,
    flags: EntityFlags,
    position: Vector3,
    velocity: Vector3,
    pitch: u8,
    yaw: u8,
}

impl Chicken {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            flags: EntityFlags::default(),
            position: Vector3::default(),
            velocity: Vector3::default(),
            pitch: 0,
            yaw: 0,
        }
    }
}
