use titanium::base::{Size, Vec4f};
use titanium::user_interface::{
    layout::CoordinateArea,
    widget::*
};
use titanium::canvas::{
    Layer,
    text::Text,
    graphics::*,
};

#[derive(Clone)]
pub struct TopBar {
    pub list: Vec<String>,
    pub height: f32,
    pub font_color: Vec4f,
    pub color: Vec4f,
    pub hover: Option<(String,Vec4f)>,
}

impl TopBar {
    pub fn new(list: Vec<String>, font_color: Vec4f, color: Vec4f, height: f32) -> Self {
        Self {
            list,
            height,
            font_color,
            color,
            hover: None,
        }
    }
    
}

impl WidgetComponent for TopBar {
    fn build(&self,coordinate_area: CoordinateArea, size: Size) -> Vec<Layer> {
        let mut layers: Vec<Layer> = Vec::new();

        let mut anchor = coordinate_area.top_left_point;
        let height = coordinate_area.bottom_right_point[1] - coordinate_area.top_left_point[1];
        let char_pixel_height = height * size.height;
        let char_width = height / 2.0;
        let char_pixel_width = char_width * size.width;

        for item in &self.list {
            let text_width = item.len() as f32 * char_width;
            let width = text_width + char_width;
            layers.push(Layer::create(
                item.clone(),
                rectangle::Rectangle::create(anchor, width, height, self.color,  GraphicsType::Polygon).to_graphics(),
                Text::create(item.clone(), 
                             [anchor[0] + char_width / 2.0 , anchor[1] ], 
                             Size { height: char_pixel_height, width: char_pixel_width}, 
                             text_width, 
                             self.font_color)
            ));
            anchor = [anchor[0] + width,anchor[1]];
        }

        layers.push(Layer::graphics(
                "top_bar".to_string(),
                rectangle::Rectangle::create(
                    coordinate_area.top_left_point, 
                    coordinate_area.bottom_right_point[0] - coordinate_area.top_left_point[0],
                    coordinate_area.bottom_right_point[1] - coordinate_area.top_left_point[1], 
                    self.color, 
                    GraphicsType::Polygon).to_graphics()));

        layers
    }
}

#[derive(Clone)]
pub struct TopBarBuilder {
    pub height: f32,
    pub color: Vec4f,
    pub font_color: Vec4f,
    pub list: Vec<String>,
}

impl TopBarBuilder {

    pub fn new(height: f32, color: Vec4f, font_color: Vec4f) -> Self {
        Self {
            height,
            color,
            font_color,
            list: Vec::new(),
        }
    }

    pub fn to_widget(&self) -> Widget {
        Widget::new(
            "top_bar".to_string(),
            Anchor::LeftTop(WindowUnit::Point([0.0,0.0])), 
            WindowUnit::Point(1.0), 
            WindowUnit::Pixel(self.height), 
            Box::new(TopBar::new(self.list.clone(), self.font_color, self.color, self.height)))
    }
}