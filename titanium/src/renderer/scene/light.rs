use crate::renderer::base::alias::Vec3f;

pub struct PointLight{
    pub position: Vec3f,
    pub color: Vec3f,
}

impl PointLight {
    fn new(position: Vec3f, color: Vec3f) -> Self {
        PointLight {
            position,
            color,
        }
    }
}