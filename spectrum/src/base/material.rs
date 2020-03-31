use super::ray::Ray;
use super::light::Light;
use super::intersect::Hit;
use math_utils::*;
use std::sync::Arc;

pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Ray,
}

impl Scatter {
    #[inline]
    pub fn new(attenuation: Color, scattered: Ray) -> {
        Self {
            attenuation,
            scattered,
        }
    }
}

pub trait Material {
    fn scatter(&self,ray:&Ray, light: &Vec<Arc<dyn Light>>, hit: &Hit, color: &Color) -> Option<Scatter>;
}