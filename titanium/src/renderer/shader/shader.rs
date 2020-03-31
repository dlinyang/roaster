//convert shader
pub trait Shader<T>{
    fn to_shader(&self) -> T;
}

pub trait ShaderSource<T> {
    fn to_shader_source(&self) -> T;
}

