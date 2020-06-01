use rmu::raw::Vec4f;
use titanium::base::Size;
use titanium::gui::{
    layout::CoordinateArea,
    widget::*,
};
use titanium::renderer::canvas::{
    Layer,
    text::Text,
    graphics::*,
};

use titanium::graphics::Rectangle;  
use titanium::event::Event;

#[derive(Clone)]
pub struct InfoBar {
    pub information: String,
    pub color: Vec4f,
    pub font_color: Vec4f,
    pub coordinate_area: CoordinateArea,
    pub window_size: Size,
}

impl InfoBar {
    pub fn new(information: String, color: Vec4f, font_color: Vec4f) -> Self {
        Self {
            information,
            color,
            font_color,
            coordinate_area: Default::default(),
            window_size: Default::default(),
        }
    }

    pub fn update_information(&mut self, information: String)  {
        self.information = information;
    }
}


impl WidgetComponent for InfoBar {
    fn build(&mut self,coordinate_area: CoordinateArea, size: Size) -> Vec<Layer> {
        let mut layers: Vec<Layer> = Vec::new();

        let height = coordinate_area.bottom_right_point[1] - coordinate_area.top_left_point[1];
        let width = coordinate_area.bottom_right_point[0] - coordinate_area.top_left_point[0];
        let char_pixel_height = height * size.height;
        let char_width = height/2.0;
        let char_pixel_width = char_width * size.width;

        layers.push(Layer::create(
            "info".to_string(), 
            Rectangle::create(
                coordinate_area.top_left_point,
                width,
                height, 
                self.color, GraphicsType::Polygon).into(),
            Text::create(self.information.clone(),
             coordinate_area.top_left_point, 
             Size { height: char_pixel_height, width: char_pixel_width}, 
             width,
             self.font_color)));
             
        layers
    }

    fn event(&mut self, event: Event) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct InfoBarBuilder {
    pub height: f32,
    pub color: Vec4f,
    pub font_color: Vec4f,
    pub information: String,
}

impl InfoBarBuilder {
    pub fn new(height: f32, color: Vec4f, font_color: Vec4f) -> Self {
        Self {
            height,
            color,
            font_color,
            information: "Status start".to_string(),
        }
    }

    pub fn to_widget(&self) -> Widget {
        Widget::new(
            "info_bar".to_string(),
            Anchor::RightBottom(WindowUnit::Point([1.0,1.0])), 
            WindowUnit::Point(1.0), 
            WindowUnit::Pixel(self.height), 
        Box::new(InfoBar::new(self.information.clone(), self.color, self.font_color)))
    }
}