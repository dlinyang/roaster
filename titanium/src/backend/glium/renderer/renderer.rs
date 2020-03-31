use crate::renderer::*;
use crate::base::Position;

use super::super::GLApplication;
use super::super::shader::Frame;
use crate::scene::camera::Camera;
use crate::scene::material::Material;

use glium::Surface;
use glium::DrawParameters;
use glium::*;

impl Renderer for GLApplication {

    fn render(&self) {

        let parameters: DrawParameters = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                write: true,
                ..Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        let mut frame = self.display.draw();

        let vertices = [Position::new([-1.0, -1.0], [0.0, 0.0]),
                        Position::new([1.0, -1.0], [1.0, 0.0]),
                        Position::new([1.0, 1.0], [1.0, 1.0]),
                        Position::new([-1.0, 1.0], [0.0, 1.0])];
        
        let vertex_buffer = glium::VertexBuffer::new(&self.display, &vertices).unwrap();

        let index_buffer = glium::IndexBuffer::new(&self.display, index::PrimitiveType::TrianglesList, &[0 as u32,1,2,0,2,3]).unwrap();

        let shader = Frame.to_shader().bind_shader(&self.display);

        let [r, g, b, a] = self.render_data.bg_color;
        frame.clear_color_and_depth((r, g, b, a), 1.0);

        if let Some(scene) = &self.frame.scene {
            frame.draw(&vertex_buffer, &index_buffer, &shader,
                &uniform! {
                    tex: scene,
                }, &parameters).unwrap();
        }

        if let Some(canvas) = &self.frame.canvas {
            frame.draw(&vertex_buffer, &index_buffer, &shader, 
                &uniform! {
                    tex: canvas,
                }, &parameters).unwrap();
        }

        frame.finish().unwrap();
    }

    fn render_canvas(&mut self) {

        let object: DrawParameters = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let text: DrawParameters = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                write: true,
                ..Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        let (width,height) = self.display.gl_window().window().inner_size().into();


        let canvas = texture::Texture2d::empty_with_format(
            &self.display,
            texture::UncompressedFloatFormat::F32F32F32F32,
            texture::MipmapsOption::NoMipmap,
            width,
            height
        ).unwrap();

        let depth = texture::DepthTexture2d::empty_with_format(
            &self.display,
            texture::DepthFormat::F32,
            texture::MipmapsOption::NoMipmap,
            width,
            height
        ).unwrap();

        let mut frame = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&self.display, &canvas, &depth).unwrap();

        frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

        if let Some(canvas_data) = &self.render_data.canvas_data {
            for layer_data in &canvas_data.layers {
                if let Some(texture) = &layer_data.texture {
                    frame.draw(&layer_data.vertex_buffer, &layer_data.indices, &layer_data.shader, 
                        &uniform! {
                            color: layer_data.color,
                            tex: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                        }, &text).unwrap();
                } else {
                    frame.draw(&layer_data.vertex_buffer, &layer_data.indices, &layer_data.shader, 
                        &uniform! {
                            color: layer_data.color,
                        }, &object).unwrap();
                }
            }

            self.frame.canvas = Some(canvas);
        }
        
    }

    fn render_scene(&mut self) {

        let object: DrawParameters = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let (width,height) = self.display.gl_window().window().inner_size().into();

        let scene = texture::Texture2d::empty_with_format(
            &self.display,
            texture::UncompressedFloatFormat::F32F32F32F32,
            texture::MipmapsOption::NoMipmap,
            width,
            height,
        ).unwrap();

        let depth = texture::DepthTexture2d::empty_with_format(
            &self.display,
            texture::DepthFormat::F32,
            texture::MipmapsOption::NoMipmap,
            width,
            height,
        ).unwrap();

        let mut frame = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&self.display, &scene, &depth).unwrap();

        frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

        if let Some(scene_data) = &self.render_data.scene_data {

            let matrix = uniforms::UniformBuffer::new(&self.display, 
                Matrix{ 
                    project: scene_data.project, 
                    view: scene_data.view
                }
            ).unwrap();

            for mesh_data in scene_data.data.values() {

                if mesh_data.render_faces {
                    match mesh_data.material {
                        Material::SimpleMaterial(color) => {

                            let uniforms = uniform!{
                                Matrix: &matrix,
                                transform: mesh_data.transform,
                                color: color,
                            };

                            frame.draw(&mesh_data.vertex_buffer, &mesh_data.faces_indices, &mesh_data.shader, &uniforms, &object).unwrap();
                        },
                    }
                } 

                if mesh_data.render_edges {

                    let uniforms = uniform! {
                        Matrix: & matrix,
                        transform: mesh_data.transform,
                        color: scene_data.edge_color,
                    };

                    frame.draw(&mesh_data.vertex_buffer, &mesh_data.edges_indices, &mesh_data.shader, &uniforms, &object).unwrap();
                }

                if mesh_data.render_points {

                }

            }

            self.frame.scene = Some(scene);
        }

    }

    fn load(&mut self, render_data: &RenderData){
        if let Some(canvas) = &render_data.canvas {
            self.render_data.canvas_data = Some(self.load_canvas(&canvas));
        }
        if let Some(scene) = &render_data.scene {
            self.render_data.scene_data = Some(self.load_scene(&scene));
        }
    }

    fn update_camera(&mut self, camera: &Camera) {
        if let Some(scene_data) = &mut self.render_data.scene_data {
            scene_data.view = camera.get_view();
        }
    }

    fn set_render_edge(&mut self, name: String, flag: bool) {
        if let Some(scene_data) = &mut self.render_data.scene_data {
            if let Some(mesh_data) = scene_data.data.get_mut(&name) {
                mesh_data.render_edges = flag;
            }
        }
    }
}

use crate::base::Mat4f;

#[derive(Copy,Clone)]
pub struct Matrix {
    pub project: Mat4f,
    pub view: Mat4f,
}

implement_uniform_block!(Matrix,project,view);