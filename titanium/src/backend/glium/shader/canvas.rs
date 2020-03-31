use crate::canvas::Canvas;
use crate::renderer::shader::Shader;
use super::ShaderSource;





impl Shader<ShaderSource> for Canvas {
    fn to_shader(&self) -> ShaderSource {
        let vertex = "#version 460
                    
                     in layout(location = 0) vec2 position;
                     in layout(location = 1) vec2 tex_coordinate;
                    
                     void main() {
                         gl_Position = vec4(position,0.0,1.0);
                    }".to_string();

        let fragment = "#version 460
                      
                        uniform vec4 color;
                      
                        out vec4 f_color;
                       
                        void main() {
                            f_color = color;
                        }".to_string();
        ShaderSource {
            vertex,
            fragment,
            geometry: None,
        }
    }
}