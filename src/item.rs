use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub value: f32,
}

impl Item {
    pub fn new(value: f32) -> Item {
        Item { value }
    }
}
