use crate::scene::material::*;
use crate::renderer::shader::Shader;
use super::ShaderSource;

impl Shader<ShaderSource> for Material {
    fn to_shader(&self) -> ShaderSource {
        match self {
            SimpleMaterial(_color) => { ShaderSource { 
                                            vertex: SIMPLE_VERTEX.to_string(),
                                            fragment: SIMPLE_FRAGMENT.to_string(),
                                            geometry: None, 
                                        }},
        }
    }
}

const SIMPLE_VERTEX: &'static str = "#version 460

                                    in layout(location = 0) vec3 position;
                                    in layout(location = 1) vec3 normal;
                                    in layout(location = 2) vec2 tex_coordinate;
                                    
                                    layout(std140) uniform Matrix {
                                        mat4 project;
                                        mat4 view;
                                    };

                                    uniform mat4 transform;
                                    
                                    void main() {
                                        vec4 pos =   view * transform * vec4(position, 1.0);
                                        gl_Position = project * pos;
                                    }";

const SIMPLE_FRAGMENT: &'static str = "#version 460
                                       
                                       out vec4 f_color;
                                       
                                       uniform vec3 color;
                                       
                                       void main() {
                                           f_color = vec4(color,1.0);
                                       }";

const BLINN_PHONG_VERTEX: &'static str ="#Version 460
                            
                                         in layout(location = 0) vec3 position;
                                         in layout(location = 0) vec3 normal;
                                         in layout(location = 0) vec2 tex_coordinate;
                                         
                                         uniform mat4 transfrom;
                                         uniform mat4 project;
                                         uniform mat4 view;

                                         void main() {
                                             
                                         }";