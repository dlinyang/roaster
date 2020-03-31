use crate::base::{Vertex, Vec3f};
use crate::scene::{object::*, mesh::Mesh};
use crate::scene::material::Material;

pub fn grid(name: String, step: f32, row: usize, column: usize, color: Vec3f) -> Object {
    let w = step * row as f32;
    let h = step * row as f32;
    let  x = -w / 2.0;
    let  y = -h / 2.0;

    let mut vertices = Vec::new();

    for i in 0..(row - 1) {
        vertices.push(Vertex::new([x + step * i as f32, y ,0.0], [0.0, 0.0, 0.0], [0.0, 0.0]));
        vertices.push(Vertex::new([x + step * i as f32, y + h, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0]));
    }

    for i in 0..(column - 1) {
        vertices.push(Vertex::new([x, y + step * i as f32, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0]));
        vertices.push(Vertex::new([x + w, y + step * i as f32, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0]));
    }

    let mut i = 0;
    let mut indices = Vec::new();
    while  i < column * row - 1 {
        indices.push([i as u32, (i+1) as u32]);
        i = i + 2;
    }

    let mesh = Mesh::new(vertices,indices,Vec::new(), Material::SimpleMaterial(color));

    Object::from(name, PrimitiveObject::MeshObject(mesh))

}