pub mod world;
use world::World;

use crate::renderer::*;
use glium::Display;
use glium::glutin;
use glium::glutin::event_loop::EventLoop;
use glium_glyph::glyph_brush::{rusttype::Font,Section};
use glium_glyph::GlyphBrush;

/*manage some info*/
pub struct Application {
    pub name: String,
    pub world: World,
    pub size: Size,
}

pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Application {
    pub fn new() -> Self {
        Application::create("Titanium White".to_string(), 1240.0, 720.0, 10.0)
    }

    //
    pub fn create(name: String, width: f32, height: f32, scale: f32) -> Self {
        Application {
            name,
            world: World::new(width, height, scale),
            size: Size{width, height},
        }
    }

    pub fn main_loop(&self,scene: scene::Scene, canvas: ui::Canvas) {

        let event_loop = EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
                                        .with_title(self.name.clone())
                                        .with_inner_size(glutin::dpi::LogicalSize::new(self.size.width as f64, self.size.height as f64));
        let cb = glutin::ContextBuilder::new().with_depth_buffer(24).with_vsync(true);
    
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
        let renderer = Renderer{display,bg_color: [1.0,1.0,1.0,1.0]};

        let scene_data = self.build_context_from_scene(&scene,&renderer,&renderer.display);
        let canvas_data = self.build_context_from_ui();

        let firacode: &[u8] = include_bytes!("../../font/FiraCode-Regular.ttf");
        let fonts = vec![Font::from_bytes(firacode).unwrap()];
        let mut glyph_brush = GlyphBrush::new(&renderer.display, fonts);

        for layer in canvas.layers {
            if let Some(text) = layer.text {
                glyph_brush.queue(Section {
                text: text.context.as_str(),
                bounds: (self.size.height,self.size.width),
                screen_position: (text.position[0],text.position[1]),
                ..Section::default()
             });
            }
        }

        event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::default();

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
            }
        
        renderer.render(&scene_data,&canvas_data,&mut glyph_brush);
        });
    }


    fn build_context_from_scene(&self,scene: &scene::Scene, renderer: &Renderer, display: &Display) -> SceneData {
        let mut data = Vec::new();
        for object in &scene.objects {
            for mesh in &object.meshes {
                let vertex_buffer = glium::VertexBuffer::new(&renderer.display, &mesh.positions).unwrap();
                let indices_data = mesh.get_indices();
                let indices = glium::IndexBuffer::new(&renderer.display, glium::index::PrimitiveType::TrianglesList, indices_data.as_slice()).unwrap();
                let program = mesh.material.bind_shader(&display);
                data.push(DrawData::new(vertex_buffer, indices, program, object.transform));
            }
        }

        SceneData {
            data,
            model: self.world.model,
            project: self.world.project,
            camera: scene.camera,
        }
    }

    fn build_context_from_ui(&self) -> CanvasData {
        CanvasData::default()
    }

}