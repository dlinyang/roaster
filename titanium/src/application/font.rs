use std::collections::HashMap;
use std::fs::read;
pub struct FontBuilder {
    pub data: HashMap<String,Vec<u8>>
}

impl FontBuilder {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self,name: String, path: String) {
        let font_byte = read(path).unwrap();
        self.data.insert(name, font_byte);
    }

    pub fn get_font_byte(&self, name: &String) -> Option<&Vec<u8>> {
        self.data.get(name).clone()
    }
}