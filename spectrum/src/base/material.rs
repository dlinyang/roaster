use super::ray::Ray;
use super::hit::HitRecord;
use super::light::Light;
use super::physics::*;
use math_utils::*;
use rand::prelude::*;
use std::sync::Arc;

pub trait Material {
    fn shader(&self,ray:&Ray, light: &Vec<Arc<dyn Light>>, hit_record: &HitRecord, color: &mut Color, attenuation: &mut Vector3, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Vector3,
}

impl Lambertian {
    pub fn new() -> Self {
        Lambertian {
            albedo: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Material for Lambertian {
    fn shader(&self,ray: &Ray, lights: &Vec<Arc<dyn Light>>, hit_record: &HitRecord, color: &mut Color, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let mut add_light = Vector3::default();

        for l in lights {
            //Σc×I×(L·N)/|L||N|
            let cosine =  dot(l.get_light(hit_record.p).normalized(), hit_record.normal.normalized());
            if cosine > 0.0 {
                add_light = add_light +  l.get_color() * cosine //* l.get_brightness(hit_record.p);
            }
        }

        let target = hit_record.p + hit_record.normal + random_in_sphere();
        *scattered = Ray::new(hit_record.p, target - hit_record.p, 0.0);
        *color = add_light ;
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Vector3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new() -> Self {
        Metal {
            albedo: Vector3::new(0.0, 0.0, 0.0),
            fuzz: 0.0,
        }
    }
}

impl Material for Metal {
    fn shader(&self, ray: &Ray, light: &Vec<Arc<dyn Light>>, hit_record: &HitRecord, color: &mut Color, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let reflected = reflect( ray.direction, hit_record.normal);
        *scattered = Ray::new(hit_record.p, 
                              reflected + self.fuzz * random_in_sphere(),
                              0.0);
        *attenuation = self.albedo;
        *color = Vector3::broadcast(1.0);
        dot(scattered.direction, hit_record.normal) > 0.0
        }
}

pub struct Dielectric {
    pub refract_coe: f32,
}

impl Dielectric {
    pub fn new() -> Self {
        Dielectric {
            refract_coe: 1.0,
        }
    }
}

impl Material for Dielectric {
    fn shader(&self, ray: &Ray, light: &Vec<Arc<dyn Light>>, hit_record: &HitRecord, color: &mut Color, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let mut outward_normal = Vector3::default();
        let reflected = reflect(ray.direction, hit_record.normal);
        let mut ni_over_nt = 0.0;
        *attenuation = Vector3::new(1.0, 1.0, 1.0);
        let mut refracted = Vector3::default();
        let mut reflect_prob = 0.0;
        let mut cosine = 0.0;
        if dot(ray.direction, hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_nt = self.refract_coe;
            cosine =  self.refract_coe * dot(ray.direction, hit_record.normal) /  ray.direction.length();
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.refract_coe;
            cosine = -dot(ray.direction, hit_record.normal) / ray.direction.length(); 
        }

        if refract(ray.direction, outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = shlick(cosine, self.refract_coe);
        } else {
            *scattered = Ray::new(hit_record.p, reflected, 0.0);
            reflect_prob = 1.0;
        }

        let mut rng = rand::thread_rng();

        if rng.gen::<f32>() < reflect_prob {
            *scattered = Ray::new(hit_record.p, reflected, 0.0); // reflection
        } else {
            *scattered = Ray::new(hit_record.p, refracted, 0.0); // refraction
        }

        true
    }
}