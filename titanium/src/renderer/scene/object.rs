use super::super::base::{Vertex, Mat4f};
use glium::program::Program;

//
pub struct Mesh {
    pub positions: Vec<Vertex>,
    pub indices: Vec<[u32;3]>,
    pub material: Program,
}

pub enum Material {Empty}

impl Mesh {
    pub fn new(positions: Vec<Vertex>, indices: Vec<[u32;3]>, material: Program) -> Self {
        Mesh {
            positions,
            indices,
            material,
        }
    }

    //triangle
    pub fn get_indices(&self) -> Vec<u32> {
        let mut result = Vec::new();
        for i in &self.indices {
            let [a, b, c] = i;
            result.push(*a);
            result.push(*b);
            result.push(*c);
        }
        result
    }
}

pub struct Object {
    pub meshes: Vec<Mesh>,
    pub transform: Mat4f,
}

impl Object {
    pub fn new(meshes: Vec<Mesh>, transform: Mat4f) -> Self {
        Object {
            meshes,
            transform,
        }
    }
}