use crate::renderer::base::alias::{Mat4f};

//the world
pub struct World{
    pub model: Mat4f,
    pub project: Mat4f,
}

impl World {
    pub fn new(width: f32, height: f32, scale: f32) -> Self{
        use std::f32::consts::PI;
        World::create(width, height, scale, PI/3.0, 1024.0, 0.1)
    }

    pub fn create(width: f32, height: f32, scale: f32, fov: f32, z_far: f32, z_near: f32) -> Self {
        let model = World::model(scale, height, width);
        let project = World::perspective(fov,z_far,z_near);
        World {
            model,
            project,
        }
    }

    //convert model from real world coordination to openGL coordination
    pub fn model(scale: f32, height: f32, width: f32) -> Mat4f {
        let ratio = height/width;

        [
            [ratio / scale, 0.0       , 0.0       , 0.0],
            [0.0          , 1.0/ scale, 0.0       , 0.0],
            [0.0          , 0.0       , 1.0/ scale, 0.0],
            [0.0          , 0.0       , 0.0       , 1.0],
        ]
    }

    //perspective matrix
    pub fn perspective(fov: f32,zf: f32, zn: f32) -> Mat4f {
        let f = 1.0/(fov/2.0).tan();

        [
            [f  , 0.0,     0.0      ,          0.0],
            [0.0, f  ,     0.0      ,          0.0],
            [0.0, 0.0, 2.0/(zn - zf), (zf+zn)/(zn - zf)],
            [0.0, 0.0,     0.0      ,          1.0],
        ]
    }

    pub fn camera(look_from: [f32;3], look_at: [f32;3],vup: [f32;3]) -> Mat4f {
        [[1.0, 0.0, 0.0, 0.0]
        ,[0.0, 1.0, 0.0, 0.0]
        ,[0.0, 0.0, 1.0, 0.0]
        ,[0.0, 0.0, 0.0, 1.0]]
    }

}