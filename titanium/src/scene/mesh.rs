use crate::base::{Vertex};
use super::material::Material;

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<[u32;2]>,
    pub faces: Vec<Vec<u32>>,
    pub material: Material,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<[u32;2]>, faces: Vec<Vec<u32>>, material: Material) -> Self {
        Mesh {
            vertices,
            edges,
            faces,
            material,
        }
    }

    //triangle
    pub fn get_faces_indices(&self) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();

        for face in &self.faces {
            if face.len() > 2 {
                let first_vertex_index_of_face = face[0];
                let mut i = 1;
                while i < face.len() - 1 {
                    result.push(first_vertex_index_of_face);
                    result.push(face[i]);
                    result.push(face[i+1]);
                    i = i + 1;
                }
            }
        }
        result
    }

    pub fn get_points_indices(&self) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();

        for x in 0..self.vertices.len() - 1 {
            result.push(x as u32);
        }

        result
    }

    pub fn get_edges_indices(&self) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();

        for edge in &self.edges {
            result.push(edge[0]);
            result.push(edge[1]);
        }

        result
    }
}