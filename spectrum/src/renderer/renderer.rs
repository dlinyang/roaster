use super::super::{scene::Scene, scene::canvas::Canvas, format::image_buff::ImageBuff, base::intersect::Hit, base::ray::Ray};
use math_utils::*;
use rand::prelude::*;
use std::f32::MAX;

pub struct Renderer{
    pub scene : Scene,
    pub canvas: Canvas,
    pub depth: usize,
}

fn ceil(a: f32) -> f32 {
    if a > 1.0 {
        1.0
    }
    else if a < 0.0 {
        0.0
    }
    else {
        a
    }
}

impl Renderer {
    pub fn new(scene: Scene, canvas: Canvas, depth: usize) -> Self {
        Renderer{
            scene,
            canvas,
            depth, // reflect number
        }
    }

    //render a picture 
    pub fn render(&mut self,image_buff: &mut ImageBuff, anti_aliasing_coe: usize) {

        let w = image_buff.width;
        let h = image_buff.height;
        let mut rng = rand::thread_rng();

        for x in 0..w {
            for y in 0..h {
                let mut pixel = Vector3::default();
                for _z in 0..anti_aliasing_coe {
                    let u = ((x as f32) + rng.gen::<f32>()) / w as f32;
                    let v = ((y as f32) + rng.gen::<f32>()) / h as f32;
                    let ray = self.scene.camera.get_ray(u, v);
                    pixel = pixel + self.color(&ray, 0); // first hit
                }
                pixel = pixel / anti_aliasing_coe as f32;
                let r = (255f32 * ceil(pixel.x)) as u8;
                let g = (255f32 * ceil(pixel.y)) as u8;
                let b = (255f32 * ceil(pixel.z)) as u8;
                image_buff.data[y * w + x] = u32::from_ne_bytes([r,g,b,255]);
            }
        }
    }

    fn color(&mut self, ray: &Ray, depth: usize) -> Color {

        let mut hit_record = HitRecord::new();
        let mut scattered = Ray::default();
        let mut attenuation = Vector3::default();
        let mut light = Vector3::default();

        if self.scene.hit(ray,  0.001, MAX, &mut hit_record) {
            // object
            if depth < self.depth && self.scene.material[hit_record.object].shader(ray, &self.scene.lights, &hit_record, &mut light, &mut attenuation, &mut scattered) {
               light + attenuation * self.color(&scattered,depth + 1)
            }
            else {
                Vector3::default()
            }
            
        } 
        else {
            // background
            let normal = ray.direction.normalized();
            let t = 0.5 * (normal.y + 1.0);
            (1.0 - t) * Vector3::broadcast(1.0) + t*Vector3::new(0.5, 0.7, 1.0)
        }
    }
}