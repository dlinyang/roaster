use math_utils::vector::*;
use std::f32;
use super::super::base::intersect::*;
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

impl Intersect for Sphere {
    fn intersect(&self,ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = dot(ray.direction, ray.direction);
        let b = dot(oc, ray.direction);
        let c = dot(oc,oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let time = temp;
                let position = ray.get_a_ray(temp);
                let normal = (position - self.center) / self.radius;
                let object = self.material;
                return Some(Hit::create(time, position, normal, object))
            }

            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let time = temp;
                let position = ray.get_a_ray(temp);
                let normal = (position - self.center) / self.radius;
                let object = self.material;
                return Some(Hit::create(time, position, normal, object))
            }
        } 

        None
    }
}