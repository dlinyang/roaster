mod renderer;
mod font;

pub use renderer::*;

use std::collections::HashMap;
use crate::base::transform::*;
use crate::canvas::*;
use crate::scene::*;
use crate::scene::object::*;
use crate::renderer::shader::Shader;
use super::*;
use font::*;
use crate::base::Id4f;
use crate::base::print_mat4;

impl GLApplication {
    pub fn load_scene(&self, scene: &Scene) -> SceneData {
        let mut data: HashMap<String,MeshData> = HashMap::new();

        for name in &scene.meshes {
            if let Some(object) = scene.data.get(name) {
                match &object.sub_objects {
                    SubObject::Atomic(primitive_object) => match primitive_object {
                        PrimitiveObject::MeshObject(mesh) => {
                            let vertex_buffer =  VertexBuffer::new(&self.display, &mesh.vertices).unwrap();
                            let edges_indices = IndexBuffer::new(&self.display, PrimitiveType::LinesList, &mesh.get_edges_indices()).unwrap();
                            let faces_indices = IndexBuffer::new(&self.display, PrimitiveType::TrianglesList, &mesh.get_faces_indices()).unwrap();
                            let shader = mesh.material.to_shader().bind_shader(&self.display);
                            let mut transform = object.transform.transform();
                            let mut parent = object.parent.clone();
                            while parent != None {
                                let parent_object = scene.data.get(&parent.unwrap()).unwrap();
                                transform = mat4_x_mat4(parent_object.transform.transform(), transform);
                                parent = parent_object.parent.clone();
                            }
                            
                            data.insert(name.clone(), MeshData::new(
                                vertex_buffer,
                                edges_indices,
                                faces_indices,
                                shader,
                                mesh.material.clone(),
                                transform,
                            ));
                        },
                        _ => (),
                    },
                    _ => (),
                }
            }
        }

        SceneData {
            data,
            project: self.world.project,
            view: Id4f,
            point_color: [1.0,1.0,0.0],
            edge_color: [1.0,0.0,0.0],
        }
    }

    pub fn load_canvas(&self,canvas: &Canvas) -> CanvasData {
        let mut layers: Vec<LayerData> = Vec::new();
        for layer in &canvas.layers {
            
            if let Some(font_name)  = &canvas.font {
                use rusttype::Font;
                if let Some(font_byte) = self.font_builder.get_font_byte(font_name) {
                    let font = Font::from_bytes(font_byte).unwrap();
                    if let Some(text) = &layer.text {
                        layers.push(load_text(text, &font, &self.display, layer.name.clone()));
                    }
                }
            }
                        
            if let Some(graphic) = &layer.graphics {
                let mut vertex_array = Vec::new();
                for vertex in &graphic.get_vertex() {
                    vertex_array.push(Position::new([vertex.position[0] * 2.0 - 1.0, (-vertex.position[1] * 2.0) + 1.0],vertex.tex_coordinate));
                }
                let vertex_buffer = glium::VertexBuffer::new(&self.display, &vertex_array).unwrap();
                let indices = glium::IndexBuffer::new(
                    &self.display, graphic.get_graphics_type().covert(), &graphic.get_indices()
                ).unwrap();
                let shader = canvas.to_shader().bind_shader(&self.display);

                layers.push(
                    LayerData { 
                        name: layer.name.clone(),
                        vertex_buffer,
                        indices,
                        texture: None,
                        shader,
                        color: graphic.get_color(),
                    }
                );
            }
            
        }
        
        layers.reverse();
        
        CanvasData {
            layers,
        }
    }
}

use crate::canvas::graphics::GraphicsType;
use glium::index::PrimitiveType;
impl GraphicsType {
    pub fn covert(&self) -> PrimitiveType {
        match self {
            GraphicsType::Points => PrimitiveType::Points,
            GraphicsType::Line => PrimitiveType::LinesList,
            GraphicsType::Loop => PrimitiveType::LineLoop,
            GraphicsType::Polygon => PrimitiveType::TrianglesList,
        }
    }
}