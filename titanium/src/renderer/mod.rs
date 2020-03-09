pub mod base;
pub mod pipeline;
pub mod scene;
pub mod ui;

use base::Vertex;
use glium::{index::*, uniform, uniforms::*, Program, VertexBuffer};
use glium::{Display, Surface};
use glium_glyph::GlyphBrush;

// renderer once render a frame. used in main loop.
pub struct Renderer {
    pub display: Display,
    pub bg_color: [f32; 4],
}

impl Renderer {
    pub fn new(display: Display, bg_color: [f32; 4]) -> Self {
        Renderer { display, bg_color }
    }

    pub fn render(&self, context: &SceneData, ui: &CanvasData,glyph_brush: &mut GlyphBrush) {
        //render a frame;
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut frame = self.display.draw();

        //set world background color;
        let [r, g, b, a] = self.bg_color;
        frame.clear_color_and_depth((r, g, b, a), 1.0);

        let light_pos: [f32; 3] = [1.0; 3];
        let light_color: [f32; 3] = [0.5; 3];

        for object_data in &context.data {
            frame
                .draw(
                    &object_data.vertex_buffer,
                    &object_data.indices,
                    &object_data.shader,
                    &uniform! {transform: object_data.transform
                    ,model: context.model
                    ,project: context.project
                    ,camera: context.camera
                    ,light_pos: light_pos
                    ,light_color: light_color},
                    &params,
                )
                .unwrap();
        }

        glyph_brush.draw_queued(&self.display,&mut frame);

        frame.finish().unwrap();
    }
}

pub struct SceneData {
    pub data: Vec<DrawData>,
    pub model: [[f32; 4]; 4],
    pub project: [[f32; 4]; 4],
    pub camera: [[f32; 4]; 4],
}

pub type CanvasData = Vec<UIData>;

pub struct UIData {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub texture: glium::texture::Texture2d,
    pub indices: IndexBuffer<u32>,
    pub shader: Program,
}

pub struct DrawData {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<u32>,
    pub shader: Program,
    pub transform: [[f32; 4]; 4],
}

impl DrawData {
    pub fn new(
        vertex_buffer: VertexBuffer<Vertex>,
        indices: IndexBuffer<u32>,
        shader: Program,
        transform: [[f32; 4]; 4],
    ) -> Self {
        DrawData {
            vertex_buffer,
            indices,
            shader,
            transform,
        }
    }
}