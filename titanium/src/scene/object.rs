use crate::base::transform::Transform;
use super::{mesh};

#[derive(Clone)]
pub enum PrimitiveObject {
    Empty,
    MeshObject(mesh::Mesh),
}

#[derive(Clone)]
pub enum SubObject {
    Atomic(PrimitiveObject),
    Discreteness{ node_names: Vec<String> },
}

#[derive(Clone)]
pub struct Object {
    pub name: String,
    pub parent: Option<String>,
    pub transform: Transform,
    pub sub_objects: SubObject,
}

impl Object {
    
    pub fn new(name: String) -> Self {
        Self {
            name,
            parent: None,
            transform: Transform::new(),
            sub_objects: SubObject::Atomic(PrimitiveObject::Empty),
        }

    }

    pub fn from(name: String, primitive_object: PrimitiveObject) -> Self {
        Self {
            name,
            parent: None,
            transform: Transform::new(),
            sub_objects: SubObject::Atomic(primitive_object),
        }
    }

}

use super::light::Light;
#[derive(Clone)]
pub struct LightObject {
    pub name: String,
    pub light: Light,
}

use super::camera::Camera;
#[derive(Clone)]
pub struct CameraObject {
    pub name: String,
    pub camera: Camera,
}