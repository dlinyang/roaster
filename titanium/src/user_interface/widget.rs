use crate::base::{Vec2f, Size};
use crate::canvas::Layer;
use super::layout::{
    CoordinateArea
};

pub struct Widget {
    pub name: String,
    pub anchor: Anchor,
    pub height: WindowUnit<f32>,
    pub width: WindowUnit<f32>,
    pub widget_component: Box<dyn WidgetComponent>,
}

impl Widget {
    pub fn new(name: String, anchor: Anchor, width: WindowUnit<f32>, height: WindowUnit<f32>, widget_component:Box<dyn WidgetComponent>) -> Self {
        Self {
            name,
            anchor,
            width,
            height,
            widget_component,
        }
    }

    pub fn build(&self, window_size: Size) -> Vec<Layer> {
        let mut layers: Vec<Layer> = Vec::new();
        let coordinate_area = self.coordinate_area(window_size);
        layers.append(&mut self.widget_component.build(coordinate_area, window_size));
        layers
    }

    pub fn coordinate_area(&self, window_size: Size) -> CoordinateArea {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut w: f32 = 0.0;
        let mut h: f32 = 0.0;

        match self.width {
            WindowUnit::Pixel(w0) => {
                w = w0 / window_size.width;
            },
            WindowUnit::Point(w0) =>{
                w = w0;
            },
        }

        match self.height {
            WindowUnit::Pixel(h0) => {
                h = h0 / window_size.height;
            },
            WindowUnit::Point(h0) => {
                h = h0;
            }
        }

        match self.anchor {

            Anchor::LeftTop(position) =>  match position {
                WindowUnit::Pixel([x0,y0]) => {
                    x = x0 / window_size.width;
                    y = y0 / window_size.height;
                },
                WindowUnit::Point([x0,y0]) => {
                    x = x0;
                    y = y0;
                },
            },

            Anchor::LeftMiddle(position) => match position {
                WindowUnit::Pixel([x0,y0]) => {
                    x = x0 / window_size.width;
                    y = y0 / window_size.height - h / 2.0;
                },
                WindowUnit::Point([x0,y0]) => {
                    x = x0;
                    y = y0 - h / 2.0;
                }
            }

            Anchor::LeftBottom(position) => match position {
                WindowUnit::Pixel([x0, y0]) => {
                    x = x0 / window_size.width;
                    y = y0 / window_size.height - h;
                },
                WindowUnit::Point([x0, y0]) => {
                    x = x0;
                    y = y0 - h;
                }
            },

            Anchor::RightTop(position) => match position {
                WindowUnit::Pixel([x0, y0]) => {
                    x = x0 / window_size.width - w;
                    y = y0 / window_size.height;
                },
                WindowUnit::Point([x0, y0]) => {
                    x = x0 - w;
                    y = y0;
                }
            },

            Anchor::RightMiddle(position) => match position {
                WindowUnit::Pixel([x0, y0]) => {
                    x = x0 / window_size.width  - w;
                    y = y0 / window_size.height - h / 2.0;
                },
                WindowUnit::Point([x0, y0]) => {
                    x = x0 - w;
                    y = y0 - h / 2.0;
                }
            },

            Anchor::RightBottom(position) =>  match position {
                WindowUnit::Pixel([x0, y0]) => {
                    x = x0 / window_size.width - w;
                    y = y0 / window_size.height - h;
                },
                WindowUnit::Point([x0, y0]) => {
                   x = x0 - w;
                   y = y0 - h; 
                }
            },

            Anchor::TopMiddle(position) => match position {
                WindowUnit::Pixel([x0, y0]) => {
                    x = x0 / window_size.width - w / 2.0;
                    y = y0 / window_size.height;
                },
                WindowUnit::Point([x0, y0]) => {
                    x = x0 - w / 2.0;
                    y = y0;
                }
            }

            Anchor::BottomMiddle(position) => match position {
                WindowUnit::Pixel([x0, y0]) => {
                    x = x0 / window_size.width - w / 2.0;
                    y = y0 / window_size.height - h;
                },
                WindowUnit::Point([x0, y0]) => {
                    x = x0 - w / 2.0;
                    y = y0 - h;
                }
            }

            Anchor::Center(position) => match position {
                WindowUnit::Pixel([x0, y0]) => {
                    x = x0 / window_size.width - w / 2.0;
                    y = y0 / window_size.height - h / 2.0;
                },
                WindowUnit::Point([x0, y0]) => {
                    x = x0 - w / 2.0;
                    y = y0 - h / 2.0;
                }
            },
        }

        CoordinateArea {
            top_left_point: [x,y],
            bottom_right_point: [x + w, y + h],
        }
    }

    pub fn in_area(&self, x: f32, y: f32, window_size: Size) -> bool {
        let coordinate_area = self.coordinate_area(window_size);
        let [x0, y0] = coordinate_area.top_left_point;
        let [x1, y1] = coordinate_area.bottom_right_point;

        if  (x0 < x) & (x < x1) & (y0 < y) & (y < y1) {
            true
        }
        else {
            false
        }
    }
}

#[derive(Copy, Clone)]
pub enum WindowUnit<T> where T: Copy + Clone{
    Pixel(T),
    Point(T),
}

#[derive(Copy,Clone)]
pub enum Anchor {
    LeftTop(WindowUnit<Vec2f>),
    LeftMiddle(WindowUnit<Vec2f>),
    LeftBottom(WindowUnit<Vec2f>),
    RightTop(WindowUnit<Vec2f>),
    RightMiddle(WindowUnit<Vec2f>),
    RightBottom(WindowUnit<Vec2f>),
    TopMiddle(WindowUnit<Vec2f>),
    BottomMiddle(WindowUnit<Vec2f>),
    Center(WindowUnit<Vec2f>),
}

pub trait WidgetComponent {
    fn build(&self, coordinate_area: CoordinateArea, size: Size) -> Vec<Layer>;
}