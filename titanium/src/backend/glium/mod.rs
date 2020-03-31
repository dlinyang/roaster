#[macro_use]
pub mod application;
pub mod shader;
pub mod event;
pub mod renderer;

pub use application::*;
pub use event::*;
pub use renderer::*;

use glium::implement_vertex;
use crate::base::vertex::{Vertex, Position};

implement_vertex!(Vertex, position, normal, tex_coordinate);
implement_vertex!(Position, position, tex_coordinate);

use glium::glutin::event_loop::EventLoop;
use glium::Display;
use glium::texture::Texture2d;

use crate::application::{
    world::World,
    window::Window, 
    font::FontBuilder};

pub struct GLApplication {
    pub name: String,
    pub world: World,
    pub window: Window,
    pub font_builder: FontBuilder,
    pub event_loop: EventLoop<()>,
    pub display: Display,
    pub render_data: GlRenderData,
    pub frame: Frame,
}

pub struct Frame {
    pub scene: Option<Texture2d>,
    pub canvas: Option<Texture2d>,
}

use crate::base::{Vec3f,Vec4f};

pub struct GlRenderData {
    pub scene_data: Option<SceneData>,
    pub canvas_data: Option<CanvasData>,
    pub bg_color: Vec4f,
}

impl GlRenderData {
    pub fn new(bg_color: Vec4f) -> Self {
        Self {
            scene_data: None,
            canvas_data: None,
            bg_color,
        }
    }
}

use crate::base::Mat4f;
use crate::scene::material::Material;
use glium::vertex::VertexBuffer;
use glium::index::IndexBuffer;
use glium::program::Program;
use std::collections::HashMap;

pub struct SceneData {
    pub data: HashMap<String,MeshData>,
    pub project: Mat4f,
    pub view: Mat4f,
    pub point_color: Vec3f,
    pub edge_color: Vec3f,
} 

//A mesh data
pub struct MeshData {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub edges_indices: IndexBuffer<u32>,
    pub faces_indices: IndexBuffer<u32>,
    pub shader: Program,
    pub material: Material,
    pub transform: Mat4f,
    pub render_points: bool,
    pub render_edges: bool,
    pub render_faces: bool,
}

impl MeshData {
    pub fn new(
        vertex_buffer: VertexBuffer<Vertex>,
        edges_indices: IndexBuffer<u32>,
        faces_indices: IndexBuffer<u32>,
        shader: Program,
        material: Material,
        transform: [[f32; 4]; 4],
    ) -> Self {
        Self {
            vertex_buffer,
            edges_indices,
            faces_indices,
            shader,
            material,
            transform,
            render_points: false,
            render_edges: false,
            render_faces: true,
        }
    }
}

pub struct CanvasData { 
    pub layers: Vec<LayerData>,
}

pub struct LayerData {
    pub name: String,
    pub vertex_buffer: VertexBuffer<Position>,
    pub texture: Option<Texture2d>,
    pub indices: IndexBuffer<u32>,
    pub shader: Program,
    pub color: [f32;4],
}