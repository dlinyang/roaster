use std::collections::HashMap;
use super::widget::Widget;

pub struct Layout{
    pub widgets: HashMap<String,Widget>,
    pub font: Option<String>,
    pub window_size: Size,
}

use crate::base::Vec2f;

pub struct CoordinateArea {
    pub top_left_point: Vec2f,
    pub bottom_right_point: Vec2f,
}

impl Default for CoordinateArea {
    fn default() -> Self{
        Self{
            top_left_point: [0.0, 0.0],
            bottom_right_point: [0.0, 0.0],
        }
    }
}

use crate::base::Size;
use crate::canvas::*;

impl Layout {
    pub fn new(window_size: Size) -> Self {
        Self {
            widgets: HashMap::new(),
            font: None,
            window_size,
        }
    }

    pub fn add_widget(&mut self, widget: Widget) {
        self.widgets.insert(widget.name.clone(), widget);
    }
    
    pub fn build(&self) -> Canvas {

        let mut canvas = Canvas::new();
        canvas.font = self.font.clone();

        for widget in self.widgets.values() {
            canvas.layers.append(&mut widget.build(self.window_size));
        }
        canvas
    }

    pub fn event(&self, x_axis: f32, y_axis: f32)  -> Option<String> {
        let x = x_axis/self.window_size.width;
        let y = y_axis/self.window_size.height;
        let mut result: Option<String> = None;
        for widget in self.widgets.values() {
            if widget.in_area(x, y, self.window_size) {
                result = Some(widget.name.clone());
                break;
            }
        }
        result
    }
}