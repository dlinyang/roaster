pub mod object;
pub mod light;

use crate::renderer::base::{Mat4f,Id4f};
use object::Object;
use light::PointLight;

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<PointLight>,
    pub camera: Mat4f,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            camera: Id4f,
        }
    }

    #[inline]
    pub fn add_object(&mut self,object: Object){
        self.objects.push(object);
    }

    #[inline]
    pub fn add_light(&mut self,light: PointLight) {
        self.lights.push(light);
    }

    #[inline]
    pub fn set_camera(&mut self,camera: Mat4f) {
        self.camera = camera;
    }
}