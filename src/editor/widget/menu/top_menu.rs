use rmu::raw::Vec4f;
use titanium::base::Size;
use titanium::gui::{
    layout::CoordinateArea,
    widget::*
};
use titanium::renderer::canvas::{
    Layer,
    text::Text,
    graphics::*,
};
use titanium::graphics::Rectangle;
use titanium::event::Event;

#[derive(Clone)]
pub struct TopMenu {
    pub name: String,
    pub list: Vec<String>,
    pub height: f32,
    pub font: String,
    pub font_color: Vec4f,
    pub color: Vec4f,
    pub font_scale: f32,
    pub hover_layer: Option<Layer>,
    pub hover_color: Vec4f,
    pub coordinate_area: CoordinateArea,
    pub window_size: Size,
}

impl TopMenu {
    pub fn new(name: String, list: Vec<String>, font: String,font_color: Vec4f, color: Vec4f, height: f32) -> Self {
        Self {
            name,
            list,
            height,
            font,
            font_color,
            color,
            font_scale: 1.0,
            hover_layer: None,
            hover_color: [color[0] / 2.0, color[1] / 2.0, color[2] / 2.0, color[3] / 2.0],
            coordinate_area: Default::default(),
            window_size: Default::default(),
        }
    }
    
}

impl WidgetComponent for TopMenu {

    fn build(&mut self,coordinate_area: CoordinateArea, size: Size) -> Vec<Layer> {

        self.coordinate_area = coordinate_area;
        self.window_size = size;

        let mut layers: Vec<Layer> = Vec::new();

        let mut anchor = coordinate_area.top_left_point;
        let height = coordinate_area.height();
        let char_pixel_height = height * size.height;
        let char_pixel_width = char_pixel_height * self.font_scale;
        let char_width =  char_pixel_height / size.width;

        for item in &self.list {
            let text_width = item.len() as f32 * char_width ;
            let width = text_width + char_width;

            layers.push(Layer::create(
                item.clone(),
                Rectangle::create(anchor, width, height, self.color,  GraphicsType::Polygon).into(),
                Text::create(item.clone(),
                             self.font.clone(),
                             [anchor[0] + char_width / 2.0 , anchor[1] ], 
                             Size { height: char_pixel_height, width: char_pixel_width}, 
                             text_width, 
                             self.font_color)
            ));
            anchor = [anchor[0] + width,anchor[1]];
        }

        layers.push(Layer::graphics(
                self.name.clone(),
                Rectangle::create(
                    coordinate_area.top_left_point, 
                    coordinate_area.width(),
                    coordinate_area.height(), 
                    self.color, 
                    GraphicsType::Polygon).into()));

        layers
    }

    fn event(&mut self, event: Event) -> Vec<Layer> {
        let mut layers: Vec<Layer> = Vec::new();

        match event {
            Event::CursorEvent{x,y} => {
                let x_axis = x / self.window_size.width;
                let y_axis = y / self.window_size.height;

                if self.coordinate_area.in_area(x_axis, y_axis) {
                    let height = self.coordinate_area.height();
                    let char_width = height * self.window_size.height * self.font_scale / self.window_size.width;

                    let mut x: f32 = 0.0;
                    let mut name = String::new();
                    let mut width: f32 = 0.0;

                    for item in &self.list {
                        width = (item.len() as f32 + 1.0) * char_width;
                        if x_axis < x + width {
                            name = item.clone();
                            break;
                        } else {
                            x = x + width;
                        }
                    }
                    
                    let layer = Layer::create(
                                    name.clone(), 
                                    Rectangle::create([x,0.0], char_width, height, self.color, GraphicsType::Polygon).into(),
                                    Text::create(name.clone(), self.font.clone(), [x,0.0], Size { height: height, width: width}, char_width, self.font_color),
                                );

                    let hovered_layer = Layer::create(
                        name.clone(), 
                        Rectangle::create([x,0.0], char_width, height, self.hover_color, GraphicsType::Polygon).into(),
                        Text::create(name.clone(), self.font.clone(), [x,0.0], Size { height: height, width: width}, char_width, self.font_color),
                    );

                    let flag = match &self.hover_layer {
                        Some(hovered) => {
                            if &hovered.name != &name {
                                layers.push(hovered.clone());
                                layers.push(hovered_layer);
                                true
                            } else {
                                false
                            }
                        },
                        None => {
                            layers.push(hovered_layer);
                            false
                        }
                    };

                    if flag {
                        self.hover_layer = Some(layer);
                    }
                }
            },
            _ => (),
        }

        layers
    }
}

#[derive(Clone)]
pub struct TopMenuBuilder {
    pub name: String,
    pub height: f32,
    pub color: Vec4f,
    pub font: String,
    pub font_color: Vec4f,
    pub list: Vec<String>,
}

impl TopMenuBuilder {

    pub fn new(name: String, height: f32, color: Vec4f, font: String, font_color: Vec4f) -> Self {
        Self {
            name,
            height,
            color,
            font,
            font_color,
            list: Vec::new(),
        }
    }

    pub fn to_widget(&self) -> Widget {
        Widget::new(
            self.name.clone(),
            Anchor::LeftTop(WindowUnit::Point([0.0,0.0])), 
            WindowUnit::Point(1.0), 
            WindowUnit::Pixel(self.height), 
            Box::new(TopMenu::new(self.name.clone(),self.list.clone(), self.font.clone(), self.font_color, self.color, self.height)))
    }
}