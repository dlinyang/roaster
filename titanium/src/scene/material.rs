use crate::base::Vec3f;

#[derive(Clone,Copy)]
pub enum Material {
    SimpleMaterial(Vec3f),
}

pub use Material::*;

pub struct Phong {
    pub ambient: Vec3f,
    pub diffuse: Vec3f,
    pub specular: Vec3f,
}