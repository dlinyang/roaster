use math_utils::*;
use crate::base::*;
use crate::base::ray::Ray;
use crate::base::hit::HitRecord;
use crate::base::physics::*;

use std::sync::Arc;

pub struct SimpleBSDF {
    pub albedo: Vector3,
    pub base_color: Vector3,
    pub fuzz: f32,
    pub refract_coe: f32,
    pub specular: f32,
}

impl SimpleBSDF {
    pub fn new() -> Self {
        SimpleBSDF {
            albedo: Vector3::new(0.1,0.1,0.1),
            base_color: Vector3::broadcast(1.0),
            fuzz: 0.1,
            refract_coe: 0.0,
            specular: 4.0,
        }
    }
}

impl Material for SimpleBSDF {
    fn shader(&self,ray: &Ray, lights: &Vec<Arc<dyn Light>>, hit_record: &HitRecord, color: &mut Color, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let mut add_light = Vector3::default();
        for l in lights {
            let cosine = dot(l.get_light(hit_record.p).normalized(), hit_record.normal.normalized());

            if cosine > 0.0 {
                add_light = add_light + l.get_color() * cosine * l.get_brightness(hit_record.p);
            }
        }

        let target = hit_record.p + hit_record.normal + random_in_sphere();
        *scattered = Ray::new(hit_record.p, target - hit_record.p, 0.0);
        *color = self.base_color * add_light ;
        *attenuation = self.albedo;
        true
    }
}