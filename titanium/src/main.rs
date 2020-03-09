#[macro_use]
use titanium::*;
use titanium::model::cube::*;
use titanium::renderer::scene::Scene;
use titanium::renderer::ui::*;
use titanium::renderer::pipeline::shader::ShaderSource;
use titanium::renderer::base::transform;
use std::f32::consts::PI;


fn main() {
    let mut application = Application::new();

    let shader0 = ShaderSource::new("shader/base.vert","shader/base.frag");
    let shader1 = ShaderSource::new("shader/base.vert","shader/base.frag");

    let mut cube0 = cube(5.0,shader0);
    let mut cube1 = cube(1.0,shader1);

    cube0.transform = transform::rotate(PI/4.0,PI/4.0,PI/4.0);
    cube1.transform = transform::position(0.1,0.0,0.0);

    let mut scene = Scene::new();
    scene.add_object(cube0);
    scene.add_object(cube1);
    let mut canvas = Canvas::new();
    canvas.add_layer(Layer::text("WO BU XIANG XIE LE".to_string()));
    application.main_loop(scene,canvas);

}
