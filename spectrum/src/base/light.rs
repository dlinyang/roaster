use math_utils::*;

pub trait Light {
    fn get_light(&self,point: Vector3) -> Vector3;
    fn get_color(&self) -> Color;
    fn get_brightness(&self,point: Vector3) -> f32;
}

pub struct PointLight {
    pub origin: Vector3,
    pub color: Color,
    pub brightness: f32,
}

impl PointLight {
    pub fn new() -> Self {
        PointLight {
            origin: Vector3::new(2f32, 0f32, 1f32),
            color: Vector3::broadcast(1f32),
            brightness: 1.0,
        }
    }

    pub fn create(origin: Vector3, color: Color, brightness: f32) -> Self {
        PointLight {
            origin,
            color,
            brightness,
        }
    }
}

impl Light for PointLight {
    //get the light direction
    fn get_light(&self, point: Vector3) -> Vector3 {
        self.origin - point
    }

    fn get_color(&self) -> Color {
        self.color
    }

    //I = dΦ/dΩ ≃ α/d²
    fn get_brightness(&self,point: Vector3) -> f32 {
        let l = 1f32/(point - self.origin).length_square();
        self.brightness * l
    }
}