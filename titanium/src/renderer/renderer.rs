use crate::scene::camera::Camera;
//the definition of Renderer
pub trait Renderer {
    fn render(&self);
    fn render_scene(&mut self);
    fn render_canvas(&mut self);
    fn load(&mut self, render_data: &RenderData);
    fn update_camera(&mut self, camera: &Camera);
    fn set_render_edge(&mut self,name: String, flag: bool);
}

use crate::canvas::Canvas;
use crate::scene::Scene;
use crate::application::world::World;

pub struct RenderData {
    pub scene: Option<Scene>,
    pub canvas: Option<Canvas>,
}