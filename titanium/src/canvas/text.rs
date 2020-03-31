use crate::base::{Vec2f,Vec4f,Size};
//
pub struct Text {
    pub context: String,
    pub position: Vec2f,
    pub size: Size,
    pub width: f32,
    pub color: Vec4f,
}

impl Text {
    pub fn new(context: String) -> Self {
        Self {
            context,
            position: [0.0,0.0],
            size: Size {width: 1.0, height:1.0},
            width: 10.0,
            color: [1.0,1.0,1.0,1.0],
        }
    }

    pub fn create(context: String, position:Vec2f, size: Size, width: f32, color: Vec4f) -> Self{
        Self {
            context,
            position,
            size,
            width,
            color,
        }
    }
}