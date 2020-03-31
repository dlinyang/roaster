pub enum TextureType {
    RGB,
    A,
}

pub struct Texture {
    pub data: Vec<u8>,
    pub width: i32,
    pub hight: i32,
}