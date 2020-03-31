use super::graphics::Graphics;
use super::text::Text;

pub struct Canvas {
    pub layers: Vec<Layer>,
    pub font: Option<String>,
}

pub enum CanvasErr {
    ExitLayer,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            font: None,
        }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    //set font
    pub fn set_font(&mut self,font: String) {
        self.font = Some(font);
    }
}

pub struct Layer{
    pub name: String,
    pub graphics: Option<Graphics>,
    pub text: Option<Text>,
}

impl Layer {

    pub fn create(name: String, graphics: Graphics, text: Text) -> Self {
        Self {
            name,
            graphics: Some(graphics),
            text: Some(text),
        }
    }

    pub fn graphics(name: String, graphics: Graphics) -> Self {
        Self {
            name,
            graphics: Some(graphics),
            text: None,
        }
    }

    pub fn text(name: String, text: Text) -> Self {
        Self {
            name,
            graphics: None,
            text: Some(text),
        }
    }
}