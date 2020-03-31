use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub width: f32,
    pub height: f32,
}

impl Config {
    pub fn load() -> Self {
        Self {
            width: 1.0,
            height: 2.0,
        }

    }

    pub fn save() {

    }
}