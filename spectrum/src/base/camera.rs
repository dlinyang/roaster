use math_utils::{Vector3,cross};
use std::f32::consts::PI;
use num::Float;
use super::ray::Ray;

pub struct Camera{
    pub origin: Vector3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub upper_left_corner: Vector3,
    pub lens_radius: f32,
    pub u: Vector3,
    pub w: Vector3,
    pub v: Vector3,
}

impl Camera {
    pub fn new(look_from: Vector3,
           look_at  : Vector3,
           vup     : Vector3,
           vfov    : f32,
           aspect  : f32,
           aperture: f32,
           focus_dist: f32) -> Camera {
           
               let theta = vfov * PI/180f32;
               let half_height = Float::tan(theta/2f32);
               let half_width  = aspect * half_height;
               let w = (look_from - look_at).normalized();
               let u = cross(vup,w).normalized();
               let v = cross(w,u);
               Camera{
                   origin: look_from,
                   upper_left_corner: look_from - half_width * focus_dist * u + half_height * focus_dist * v - focus_dist * w,
                   horizontal: 2f32 * half_width * focus_dist * u,
                   vertical: 2f32*half_height * focus_dist * v,
                   lens_radius: aperture/2f32,
                   w: w,
                   u: u,
                   v: v, 
               }

    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, 
                 self.upper_left_corner + u * self.horizontal - v * self.vertical  - self.origin,
                 0.0)
    }

    pub fn default() -> Self {
        Camera {
            origin: Vector3::new(10.0, 2.0, 0.0),
            horizontal: Vector3::new(0.0, 4.0, 0.0),
            vertical: Vector3::new(0.0, 0.0, -2.0),
            upper_left_corner: Vector3::new(0.0, -2.0, 1.0),
            lens_radius: 1.0,
            u: Vector3::new(-1.0,0.0,0.0),
            v: Vector3::new(0.0,0.0,0.0),
            w: Vector3::new(0.0,0.0,0.0),
        }
    }
}