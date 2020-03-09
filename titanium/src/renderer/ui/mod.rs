pub mod primetype;
pub mod text;

pub use text::Text;

pub struct Canvas {
    pub layers: Vec<Layer>,
}
impl Canvas {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }
}
pub struct Layer{
    //graphics: Option<>,
    pub text: Option<Text>,
}

impl Layer {
    pub fn text(context: String) -> Self {
        Self {
            text: Some(Text::new(context))
        }
    }
}