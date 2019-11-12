use glium::Program;
use crate::renderer::base::{Vertex, alias::Id4f};
use crate::renderer::scene::object::{Mesh,Object};

pub fn cube(length: f32, program: Program) -> Object {
    let half_l = length / 2.0;
    let a1 = Vertex::new([half_l, half_l,  half_l], [half_l, half_l,  half_l], [0.0, 0.0]);
    let a2 = Vertex::new([-half_l, half_l, half_l], [-half_l, half_l, half_l], [0.0, 0.0]);
    let a3 = Vertex::new([half_l, -half_l, half_l], [half_l, -half_l, half_l], [0.0, 0.0]);
    let a4 = Vertex::new([-half_l, -half_l, half_l], [-half_l, -half_l, half_l], [0.0, 0.0]);
    let a5 = Vertex::new([half_l, half_l, -half_l],  [half_l, half_l, -half_l], [0.0, 0.0]);
    let a6 = Vertex::new([-half_l, half_l, -half_l], [-half_l, half_l, -half_l], [0.0, 0.0]);
    let a7 = Vertex::new([half_l, -half_l, -half_l], [half_l, -half_l, -half_l], [0.0, 0.0]);
    let a8 = Vertex::new([-half_l, -half_l, -half_l], [-half_l, -half_l, -half_l], [0.0, 0.0]);

    let mesh = Mesh::new(vec![a1,a2,a3,a4,a5,a6,a7,a8], vec![[0,1,2], [1,2,3],
                                                             [0,1,4], [1,4,5],
                                                             [1,3,5], [3,5,7],
                                                             [2,3,6], [3,6,7],
                                                             [0,2,4], [2,4,6],
                                                             [4,5,6], [5, 6,7]], program);
                                                        
    Object::new(vec!(mesh),Id4f)
} 