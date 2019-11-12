#[macro_use]

use glium::*;
use super::alias::{Vec2f, Vec3f};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Vec3f,
    pub normal: Vec3f,
    pub tex_coordinate: Vec2f,
}

implement_vertex!(Vertex, position, normal, tex_coordinate);

impl Vertex {
    #[inline]
    pub fn new(position: Vec3f, normal: Vec3f, tex_coordinate: Vec2f) -> Self {
        Vertex {
            position,
            normal,
            tex_coordinate,
        }
    }
}

//use for 2d and ui
#[derive(Copy,Clone)]
pub struct Position {
    pub position: Vec2f,
    pub tex_coordinate: Vec2f,
}

implement_vertex!(Position, position, tex_coordinate);

impl Position {
    #[inline]
    pub fn new(position: Vec2f, tex_coordinate: Vec2f) -> Self {
        Position {
            position,
            tex_coordinate,
        }
    }

    #[inline]
    pub fn from(position: Vec2f) -> Self {
        Position {
            position,
            tex_coordinate: [0.0, 0.0],
        }
    }
}