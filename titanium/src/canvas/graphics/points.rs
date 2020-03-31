use crate::base::{Vec2f,Vec4f, Position};
use super::{ToGraphics, Graphics, GraphicsType};

#[derive(Clone)]
pub struct Points {
    pub point_array: Vec<Vec2f>,
    pub color: Vec4f,
}

impl Points {
    pub fn new() -> Self {
        Self {
            point_array: vec![[0.0,0.0]],
            color: [0.5,0.5,0.5,1.0],
        }
    }

    pub fn create(point_array: Vec<Vec2f>, color: Vec4f) -> Self {
        Self {
            point_array,
            color,
        }
    }

    pub fn to_graphics(&self) -> Graphics {
        Graphics {
            positions: self.get_position(),
            color: self.color,
            graphics_type: GraphicsType::Points,
        }
    }

    pub fn get_position(&self) -> Vec<Position> {

        let mut result = Vec::new();

        for x in &self.point_array {
            result.push(Position::from(*x));
        }
        result
    }

}