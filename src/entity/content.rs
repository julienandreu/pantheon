use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Content {
    pub name: String,
    pub description: String,
    pub image: String,
}

impl Content {
    pub fn new(name: String, description: String, image: String) -> Self {
        Self {
            name,
            description,
            image,
        }
    }
}
