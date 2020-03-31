use crate::base::{Vec3f,Mat4f};
use crate::base::transform::camera;

#[derive(Copy,Clone)]
pub struct Camera {
    pub look_from: Vec3f,
    pub look_at: Vec3f,
    pub vup: Vec3f,
}

impl Camera {
    pub fn new(look_from: Vec3f, look_at: Vec3f, vup: Vec3f) -> Self {
        Self {
            look_from,
            look_at,
            vup,
        }
    }

    #[inline]
    pub fn get_view(&self) -> Mat4f {
        camera(self.look_from, self.look_at, self.vup)
    }
}