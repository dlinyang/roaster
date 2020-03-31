//
use crate::scene::*;
pub struct Format<'a> {
    pub name: String,
    pub scene: &'a Scene,
}

use std::fs::File;
use std::io::prelude::*;

impl<'a> Format<'a> { 
    pub fn save(&self) {
        if let Ok(file) = File::open(self.name.clone()) {

        } else {
            let mut file = File::create(self.name.clone());
        }
    }
}