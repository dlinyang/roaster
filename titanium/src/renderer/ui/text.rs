use crate::renderer::base::alias::Vec2f;
//
pub struct Text {
    pub context: String,
    pub position: Vec2f,
    pub scale: f32,
}

impl Text {
    pub fn new(context: String) -> Self {
        Self {
            context,
            position: [0.0,0.0],
            scale: 1.5,
        }
    }
}