use crate::renderer::base::{Position,alias::{Vec2f,Vec4f}};
use std::f32::consts::PI;

// anchor is left lower point of widget
// w h i ratio of parent widget's with and height
// for top widget is the window
pub struct Rectangle {
    pub anchor: Vec2f,
    pub w: f32,
    pub h: f32,
    pub color: Vec4f,
}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle {
            anchor: [0.9,1.0],
            w: 0.1,
            h: 0.05,
            color: [0.1, 0.1, 0.5, 1.0],
        }
    }

    pub fn get_positions(&self) -> Vec<Position> {
        rectangle(self.anchor,self.w,self.h)
    }
}

pub struct RoundRectangle {
    pub anchor: [f32;2],
    pub w: f32,
    pub h: f32,
    pub radius: f32,
    pub color: [f32;4],
}

impl RoundRectangle {
    pub fn new() -> Self {
        RoundRectangle {
            anchor: [0.9,0.9],
            w: 0.1,
            h: 0.05,
            radius: 0.01,
            color: [0.1, 0.1, 0.5, 1.0],
        }
    }

    pub fn get_positions(&self) ->  Vec<Position> {
        let [x,y] = self.anchor;
        let w = self.w;
        let h = self.h;
        let radius = self.radius;
        let mut pos: Vec<Position> = Vec::new();
        pos.append(&mut rectangle([x + radius, y - radius], w - 2.0 * radius, h - 2.0 * radius));
        pos.append(&mut rectangle([x,y - radius], radius , h - 2f32 * radius));
        pos.append(&mut rectangle([x + radius, y], w - 2f32 * radius, radius));
        pos.append(&mut rectangle([x + w - radius, y - radius], radius, h - 2f32 * radius));
        pos.append(&mut rectangle([x + radius, y - h + radius], w - 2.0 * radius, radius));
        
        pos.append(&mut angle(PI/2.0,PI/2.0,self.radius,[x + radius, y - radius], 5));
        pos.append(&mut angle(0.0,PI/2.0,self.radius,[x + w - radius, y - radius], 5));
        pos.append(&mut angle(0.0, -PI/2.0,self.radius,[x + w - radius, y - h + radius], 5));
        pos.append(&mut angle(PI,PI/2.0,self.radius,[x + radius, y - h + radius], 5));
        
        pos
    }
}

// get the point of angle in a position
fn angle(radian:f32 , delta: f32, radius: f32 ,position: [f32;2], count: usize) -> Vec<Position> {
    let [x,y] = position;
    let r_delta = delta / count as f32;
    let mut ra = radian;
    let mut pos: Vec<Position> = Vec::new();
    for _ in 0..count {
        pos.push(Position::from([x,y]));
        pos.push(Position::from([x + ra.cos() * radius, y + ra.sin() * radius]));
        ra = ra + r_delta;
        pos.push(Position::from([x + ra.cos() * radius, y + ra.sin() * radius]));
    }
    
    pos
}

fn rectangle(position: [f32;2], w: f32, h: f32) -> Vec<Position> {
    let [x,y] = position;
    let a1 = Position::from(position);
    let a2 = Position::from([x + w, y]);
    let a3 = Position::from([x, y - h]);
    let a4 = Position::from([x + w,y - h]);
    vec![a1,a2,a3,a4]
}