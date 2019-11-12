use ultraviolet_ray::format::{image_buff::ImageBuff,image_file::ImageFile};
use math_utils::Vector3;
use ultraviolet_ray::object::sphere::Sphere;
use ultraviolet_ray::scene::{Scene, canvas::Canvas};
use ultraviolet_ray::base::camera::Camera;
use ultraviolet_ray::base::material::*;
use ultraviolet_ray::renderer::*;

use std::sync::Arc;

use rand::prelude::*;

fn main() {

    println!("This is a Ray Tracing program");

    let w: usize = 800;
    let h: usize = 400;
    let s: usize = 100;
    let mut image = ImageBuff::create(w,h);
    let canvas = Canvas::new(w, h, 1.0, 2.0);

    let look_from = Vector3::new(2.5, 1.0, 10.0);
    let look_at   = Vector3::new(2.5, 1.4, -1.0);
    let dist_to_focus = (look_from - look_at).length();
    let camera = Camera::new(look_from,
                             look_at,
                             Vector3::new(0.0, 1.0, 0.0), 20.0,  w as f32/ h as f32, 2.0, dist_to_focus);
    
    let sphere0 = Sphere::new(0.5, Vector3::new(2.5, 0.0, -1.0), 20);
    let sphere1 = Sphere::new(1000.0, Vector3::new(2.5, -1000.5, -1.0), 20);
    let sphere2 = Sphere::new(0.5, Vector3::new(1.5, 0.0, -1.0), 21);
    let sphere3 = Sphere::new(0.5, Vector3::new( 3.5, -0.0, -1.0), 22);
    let mut material0 = Lambertian::new();
    material0.albedo = Vector3::new(0.1, 0.2, 0.5);
    let mut material1 = Metal::new();
    material1.albedo=  Vector3::new(0.8, 0.6, 0.2);
    material1.fuzz = 0.02;
    let mut material2 = Dielectric::new();
    material2.refract_coe = 1.5;
    let mut scene = Scene::new();
    let mut rng = rand::thread_rng();
    for x in 0..20 {
        let choose: f32 = rng.gen();
        let center = Vector3::new(5.0 * rng.gen::<f32>(), 0.2, 4.0 * rng.gen::<f32>());
        if choose < 0.8 {
            let mut material = Lambertian::new();
            material.albedo = Vector3::new(rng.gen::<f32>() * rng.gen::<f32>(),
                                           rng.gen::<f32>() * rng.gen::<f32>(),
                                           rng.gen::<f32>() * rng.gen::<f32>());
            let sphere = Sphere::new(0.2,center,x);
            scene.add_object(Arc::new(sphere));
            scene.add_material(Arc::new(material));
        }
        else if choose < 0.95 {
            let mut material = Metal::new();
            material.albedo = Vector3::new(rng.gen(),
                                           rng.gen(),
                                           rng.gen());
            material.fuzz = 0.5 * rng.gen::<f32>();
            let sphere = Sphere::new(0.2,center,x);
            scene.add_object(Arc::new(sphere));
            scene.add_material(Arc::new(material));
        }
        else {
            let mut material = Dielectric::new();
            material.refract_coe = 2.0 * rng.gen::<f32>();
            let sphere = Sphere::new(0.2,center,x);
            scene.add_object(Arc::new(sphere));
            scene.add_material(Arc::new(material));
        }
    }
    scene.add_object(Arc::new(sphere0));
    scene.add_object(Arc::new(sphere1));
    scene.add_object(Arc::new(sphere2));
    scene.add_object(Arc::new(sphere3));
    scene.add_material(Arc::new(material0));
    scene.add_material(Arc::new(material1));
    scene.add_material(Arc::new(material2));
    scene.set_camera(camera);
    let mut renderer = Renderer::new(scene, canvas, s);

    renderer.render(&mut image, 50);

    let mut file = ImageFile::new("test.png");
    file.add_image_buff(image);
    file.write_rgba();
}