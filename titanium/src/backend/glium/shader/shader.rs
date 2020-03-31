use std::fs::File;
use std::io::Read;

use glium::program::{Program, Binary};
use glium::Display;

//load shader source code
pub struct ShaderSource{
    pub vertex: String,
    pub fragment: String,
    pub geometry: Option<String>,
}

impl ShaderSource{
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