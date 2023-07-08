#[derive(Debug, Clone)]
pub struct ItemInstance {
    pub id: i16,
    pub count: i8,
    pub metadata: i16,
}

impl ItemInstance {
    pub fn new(id: i16, count: i8, metadata: i16) -> Self {
        Self {
            id,
            count,
            metadata,
        }
    }
}
