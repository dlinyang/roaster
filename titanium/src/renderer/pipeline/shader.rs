use std::fs::File;
use std::io::Read;

use glium::program::{Program, Binary};
use glium::Display;

//load shader source code
pub struct ShaderSource{
    vertex: String,
    fragment: String,
    geometry: Option<String>,
}

impl ShaderSource{
    pub fn new(vertex_shader_file_path: &str, fragment_shader_file_path: &str) -> Self {
        //read vertex shader src file 
        let mut vertex = String::new();
        File::open(vertex_shader_file_path).unwrap().read_to_string(&mut vertex).unwrap();
        //read fragment shader src file
        let mut fragment = String::new();
        File::open(fragment_shader_file_path).unwrap().read_to_string(& mut fragment).unwrap();
    
        ShaderSource {
            vertex,
            fragment,
            geometry: None,
        }
    }

    //load shader program to gpu
    #[inline]
    pub fn bind_shader(&self,display: &Display) -> Program {
        if let Some(geometry) = &self.geometry { 
            Program::from_source(display, self.vertex.as_str(), self.fragment.as_str(), Some(geometry.as_str())).unwrap()
        } else {
            Program::from_source(display, self.vertex.as_str(), self.fragment.as_str(), None).unwrap()
        }
    }

    //get tne binary code 
    #[inline]
    pub fn get_binary(&self,display: &Display) -> Binary {
        self.bind_shader(display).get_binary().unwrap()
    }
}

pub fn shader_laod(shader_name: &str) -> Program {
    // if binary code exist then load binary code else compile the source file and load
    unimplemented!()
}