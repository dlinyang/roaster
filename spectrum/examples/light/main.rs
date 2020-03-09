use math_utils::Vector3;
use spectrum::scene::{Scene, canvas::Canvas};
use spectrum::*;
use spectrum::material::*;

use std::sync::Arc;

fn main() {

    println!("This is a Ray Tracing program");

    let w: usize = 400;
    let h: usize = 200;
    let s: usize = 100;
    let mut image = ImageBuff::create(w,h);
    let canvas = Canvas::new(w, h, 2.0, 1.0);

    let look_from = Vector3::new(10f32, 0.0, 2.0);
    let look_at   = Vector3::new(0.0 , 0.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let camera = Camera::new(look_from,
                             look_at,
                             Vector3::new(0.0, 0.0, 1.0), 20.0,  w as f32/ h as f32, 2.0, dist_to_focus);
    
    let sphere0 = Sphere::new(1.0, Vector3::new(0.0, 0.0, 0.0), 0);
    let sphere1 = Sphere::new(1000.0, Vector3::new(0.0, 0.0, -1001.0), 0);
    let light = PointLight::create(Vector3::new(5f32, 5f32, 5f32),Vector3::new(1.0, 1.0, 1.0), 1.0 );
    let mut material = SimpleBSDF::new();
    material.albedo = Vector3::new(0.1, 0.2, 0.3);
    material.base_color = Vector3::new(1.0, 0.0, 0.0);
    let mut material2 = Dielectric::new();
    material2.refract_coe = 1.5;
    let mut scene = Scene::new();
    scene.add_object(Arc::new(sphere0));
    scene.add_object(Arc::new(sphere1));
    scene.add_material(Arc::new(material));
    scene.set_camera(camera);
    scene.add_light(Arc::new(light));

    let mut renderer = Renderer::new(scene, canvas, s);

    renderer.render(&mut image, 50);

    let mut file = ImageFile::new("image_out/light.png");
    file.add_image_buff(image);
    file.write_rgba();
}