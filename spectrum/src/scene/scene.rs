use crate::{base::ray::Ray, base::intersect::*, base::material::Material,base::camera::Camera, base::light::Light };
use crate::material::background::*;

use math_utils::*;
use std::sync::Arc;

pub struct Scene{
    pub objects: Vec<Arc<dyn Hit>>,
    pub material: Vec<Arc<dyn Material>>,
    pub background: Arc<dyn Material>,
    pub lights: Vec<Arc<dyn Light>>,
    pub camera: Camera,
}

impl Scene {

    pub fn new() -> Self {
        Scene{
            objects: Vec::new(),
            material: Vec::new(),
            background: Arc::new(PureColor::new()),
            lights: Vec::new(),
            camera: Camera::default(),
        }
    }

    pub fn add_object(&mut self,hit: Arc<dyn Hit>) {
        self.objects.push(hit);
    }

    pub fn add_material(&mut self, material: Arc<dyn Material>) {
        self.material.push(material);
    }

    pub fn add_light(&mut self,light: Arc<dyn Light>) {
        self.lights.push(light);
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
                   let mut temp_record = Hit::new();
                   let mut swap = t_max;
                   let x =  self.objects.len();
                   for i in 0..x { 
                       if let Some(hit) = self.objects[i].hit(ray, t_min, swap) {
                           hit = true;
                           swap = temp_record.t;
                           hit_record.normal = temp_record.normal;
                           hit_record.p = temp_record.p;
                           hit_record.t = temp_record.t;
                           hit_record.object = temp_record.object;
                       }
                   }
                   hit
               }
}