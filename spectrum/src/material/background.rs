use math_utils::*;
use crate::base::*;
use crate::base::ray::Ray;
use crate::base::intersect::Hit;
use crate::base::material::*;

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
    fn scatter(&self, ray: &Ray, light: &Vec<Arc<dyn Light>>, hit: &Hit, color: &Color) -> Option<Scatter> {
        Some(Scatter::new(self.color,Ray::default()))
    }
}