use math_utils::Vector3;
use super::ray::Ray;

pub struct HitRecord{
    pub t: f32,
    pub p: Vector3, //position
    pub normal: Vector3,
    pub object: usize,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: 0.0,
            p: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            object: 0,
        }
    }
}

pub trait Hit { 
    fn hit(&self, ray: &Ray,
                  t_min: f32,
                  t_max: f32,
                  hit_record: &mut HitRecord) -> bool; // out color
}