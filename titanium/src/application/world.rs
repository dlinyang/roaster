use crate::base::alias::{Mat4f, Vec4f};
use crate::base::transform::{perspective};

//the world
#[derive(Clone,Copy)]
pub struct World{
    pub project: Mat4f,
    pub background: Vec4f,
    pub fov: f32,
    pub z_far: f32,
    pub z_near: f32,
}

impl World {
    pub fn new(width: f32, height: f32) -> Self{
        use std::f32::consts::PI;
        World::create(width, height, PI/3.0, 1024.0, 0.1)
    }

    pub fn create(width: f32, height: f32, fov: f32, z_far: f32, z_near: f32) -> Self {
        let project = perspective(fov,z_far,z_near, height/width);
        World {
            project,
            background: [1.0, 1.0, 1.0, 1.0],
            fov,
            z_far,
            z_near,
        }
    }

    pub fn rebuild(&mut self, width: f32, height:f32) {
        self.project = perspective(self.fov, self.z_far, self.z_near, height/width);
    }
}