use math_utils::Vector3;
use super::ray::Ray;

pub struct Hit{
    pub time: f32,
    pub position: Vector3, //position
    pub normal: Vector3,
    pub object: usize,
}

impl Hit {
    #[inline]
    pub fn new() -> Self {
        Self {
            time: 0.0,
            position: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            object: 0,
        }
    }

    #[inline]
    pub fn create(time: f32, position: Vector3, normal: Vector3, object: usize) -> Self {
        Self {
            time,
            position,
            normal,
            object,
        }
    }
}

pub trait Intersect { 
    fn intersect(&self, ray: &Ray,t_min: f32,t_max: f32) -> Option<Hit>; // out color
}