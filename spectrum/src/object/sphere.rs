use math_utils::vector::*;
use std::f32;
use super::super::base::hit::*;
use super::super::base::ray::Ray;

pub struct Sphere{
    pub center: Vector3,
    pub radius: f32,
    pub material: usize,
}

impl Sphere {
    pub fn new(radius: f32,center: Vector3,material: usize) -> Self{
        Sphere{
            center,
            radius,
            material,
        }
    }

    pub fn set_material(&mut self,material: usize) {
        self.material = material;
    }
}

impl Hit for Sphere {
    fn hit(&self,ray: &Ray,
                 t_min: f32,
                 t_max: f32,
                 hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = dot(ray.direction, ray.direction);
        let b = dot(oc, ray.direction);
        let c = dot(oc,oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = ray.get_a_ray(temp);
                hit_record.normal = (hit_record.p - self.center) / self.radius;
                hit_record.object = self.material;
                return true
            } 
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = ray.get_a_ray(temp);
                hit_record.normal = (hit_record.p - self.center) / self.radius;
                hit_record.object = self.material;
                return true
            }
        } 

        false
    }
}