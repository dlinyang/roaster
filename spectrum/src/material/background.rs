use math_utils::*;
use crate::base::*;
use crate::base::ray::Ray;
use crate::base::hit::HitRecord;
use crate::base::physics::*;

use std::sync::Arc;

pub struct PureColor {
    color: Color,
}

impl PureColor {
    pub fn new() -> Self {
        PureColor {
            color: Vector3::new(0.1,0.1,0.5),
        }
    }
}

impl Material for PureColor {
    fn shader(&self, ray: &Ray, light: &Vec<Arc<dyn Light>>, hit_record: &HitRecord, color: &mut Color, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        true
    }
}