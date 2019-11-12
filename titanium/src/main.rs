#[macro_use]
use titanium_white::*;
use titanium_white::model::cube::*;
use titanium_white::renderer::scene::Scene;
use titanium_white::renderer::pipeline::shader::ShaderSource;
use titanium_white::renderer::base::transform;
use std::f32::consts::PI;

fn main() {
    let mut application = Application::new();

    let shader0 = ShaderSource::new("shader/base.vert","shader/base.frag");
    let shader1 = ShaderSource::new("shader/base.vert","shader/base.frag");
    let program0 = shader0.bind_shader(&application.renderer.display);
    let program1 = shader1.bind_shader(&application.renderer.display);

    let mut cube0 = cube(5.0,program0);
    let mut cube1 = cube(1.0,program1);

    cube0.transform = transform::rotate(PI/4.0,PI/4.0,PI/4.0);
    cube1.transform = transform::position(0.1,0.0,0.0);

    let mut scene = Scene::new();
    scene.add_object(cube0);
    scene.add_object(cube1);
    let sceneData = application.build_context_from_scene(scene);
    application.main_loop(&sceneData);

}
