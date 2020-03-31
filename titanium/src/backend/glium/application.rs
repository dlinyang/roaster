use crate::application::*;

use glium::glutin::{
    event_loop::EventLoop,
    window::WindowBuilder,
    ContextBuilder,
    dpi::LogicalSize};

use super::*;

impl ApplicationBackend<GLApplication> for Application {
    //
    fn init(&self) -> GLApplication {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_title(self.name.clone())
            .with_inner_size(LogicalSize::new(self.window.size.width as f64, self.window.size.height as f64))
            .with_decorations(self.window.decoration)
            .with_resizable(self.window.resizable);
        let context_builder = ContextBuilder::new().with_depth_buffer(24).with_vsync(self.window.v_sync);
        let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

        GLApplication {
            name: self.name.clone(),
            world: self.world,
            window: self.window.clone(),
            font_builder: font::FontBuilder::new(),
            event_loop,
            display,
            render_data: GlRenderData::new(self.world.background),
            frame: Frame {
                scene: None,
                canvas: None,
            },
        }
    }
}