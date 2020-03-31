///The basic module include some basic math type and function
pub mod alias;
pub mod vertex;
pub mod transform;
pub mod color;

pub use vertex::{Vertex,Position};
pub use alias::*;


#[derive(Clone,Copy)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
        }
    }
}