use crate::renderer::shader::Shader;
use super::ShaderSource;

pub struct Frame;

impl Shader<ShaderSource> for Frame {
    fn to_shader(&self) -> ShaderSource {
        ShaderSource {
            vertex: FRAME_VERTEX.to_string(),
            fragment: FRAME_FRAGMENT.to_string(),
            geometry: None,
        }
    }
}


const FRAME_VERTEX: &'static str = "#version 460

                                    in layout(location = 0) vec2 position;
                                    in layout(location = 1) vec2 tex_coordinate;

                                    out vec2 v_tex_coordinate;
                                    
                                    void main() {
                                        gl_Position = vec4(position,0.0,1.0);
                                        v_tex_coordinate = tex_coordinate;
                                    }";

const FRAME_FRAGMENT: &'static str = "#version 460

                                      uniform sampler2D tex;
                                     
                                      in vec2 v_tex_coordinate;
                                     
                                      out vec4 f_color;
                                     
                                      void main() {
                                          f_color = texture(tex, v_tex_coordinate);
                                      }";