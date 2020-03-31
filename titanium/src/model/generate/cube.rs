use crate::base::Vertex;
use crate::scene::{object::*, mesh::Mesh};
use crate::scene::material::Material;

pub fn cube(name: String, length: f32, material: Material) -> Object {
    let half_l = length / 2.0;
    let a1 = Vertex::new([half_l, half_l,  half_l], [half_l, half_l,  half_l], [0.0, 0.0]);
    let a2 = Vertex::new([-half_l, half_l, half_l], [-half_l, half_l, half_l], [0.0, 0.0]);
    let a3 = Vertex::new([-half_l, -half_l, half_l], [-half_l, -half_l, half_l], [0.0, 0.0]);
    let a4 = Vertex::new([half_l, -half_l, half_l], [half_l, -half_l, half_l], [0.0, 0.0]);
    let a5 = Vertex::new([half_l, half_l, -half_l],  [half_l, half_l, -half_l], [0.0, 0.0]);
    let a6 = Vertex::new([-half_l, half_l, -half_l], [-half_l, half_l, -half_l], [0.0, 0.0]);
    let a7 = Vertex::new([-half_l, -half_l, -half_l], [-half_l, -half_l, -half_l], [0.0, 0.0]);
    let a8 = Vertex::new([half_l, -half_l, -half_l], [half_l, -half_l, -half_l], [0.0, 0.0]);

    let mesh = Mesh::new(vec![a1,a2,a3,a4,a5,a6,a7,a8],
                         vec![[0,1],[1,2],[2,3],[3,0],[4,5],[5,6],[6,7],[7,4],[0,4],[1,5],[2,6],[3,7]],
                         vec![vec![0,1,2,3], vec![4,5,6,7], vec![0,1,5,4], vec![1,2,6,5], vec![2,3,7,6], vec![3,0,4,7]],
                         material);
                                                        
    Object::from(name, PrimitiveObject::MeshObject(mesh))
} 