use crate::base::ray::Ray;
use super::bvh::BVHNode;

//aixs aliasing bounding box
pub struct AABB {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub z_min: f32,
    pub z_max: f32,
}

impl AABB {
    pub fn new(x_min: f32, x_max: f32, y_min: f32,y_max: f32, z_min: f32,z_max: f32) -> Self {
        AABB {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }
}

// xₘᵢₙ - α < at < xₘₐₓ - α
#[inline]
fn compare(origin: f32, direction: f32,min: f32,max :f32) -> bool {
    let d = 1f32 / direction;
    let t0 = (min - origin) * d;
    let t1 = (max - origin) * d;

    t0 > t1
}

impl BVHNode for AABB {
    fn intersect(&self, ray: &Ray) -> bool {

        if compare(ray.origin.x, ray.direction.x, self.x_min, self.x_max) {
            false
        }
        else if compare(ray.origin.y, ray.direction.y, self.y_min, self.y_max) {
            false
        }
        else if compare(ray.origin.z, ray.direction.z, self.z_min, self.z_max) {
            false
        }
        else {
            true
        }
    }
}