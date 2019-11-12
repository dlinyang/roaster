pub mod world;
use world::World;

use crate::renderer::*;
use glium::glutin;
use glium::glutin::EventsLoop;

pub struct Application {
    pub name: String,
    pub world: World,
    pub events_loop: EventsLoop,
    pub renderer: Renderer,
}

impl Application {
    pub fn new() -> Self {
        Application::create("Titanium White".to_string(), 1240.0, 720.0, 10.0)
    }

    //
    pub fn create(name: String, width: f32, height: f32, scale: f32) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new()
                                        .with_title(name.clone())
                                        .with_dimensions(glutin::dpi::LogicalSize::new(width as f64, height as f64));
        let cb = glutin::ContextBuilder::new().with_depth_buffer(24).with_vsync(true);
        let display = glium::Display::new(wb, cb, &events_loop).unwrap();

        Application {
            name,
            world: World::new(width, height, scale),
            renderer: Renderer::new(display,[1.0, 1.0, 1.0, 1.0]),
            events_loop,
        }
    }

    pub fn build_context_from_scene(&self,scene: scene::Scene) -> SceneData {
        let mut data = Vec::new();
        for object in scene.objects {
            for mesh in object.meshes {
                let vertex_buffer = glium::VertexBuffer::new(&self.renderer.display, &mesh.positions).unwrap();
                let indices_data = mesh.get_indices();
                let indices = glium::IndexBuffer::new(&self.renderer.display, glium::index::PrimitiveType::TrianglesList, indices_data.as_slice()).unwrap();
                data.push(DrawData::new(vertex_buffer, indices, mesh.material, object.transform));
            }
        }

        SceneData {
            data,
            model: self.world.model,
            project: self.world.project,
            camera: scene.camera,
        }
    }

    pub fn main_loop(&mut self,context: &SceneData) {
        let mut closed = false;
        while !closed {

            self.events_loop.poll_events(|ev| {
                match ev {
                   glutin::Event::WindowEvent {event,..} => match event {
                       glutin::WindowEvent::CloseRequested => closed = true,
                       _ => (),
                   },
                   _ => (),
                }
            });
            
            self.renderer.render(context);
        }
    } 
}