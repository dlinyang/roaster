use crate::base::alias::Vec3f;

//
#[derive(Copy,Clone)]
pub enum Light {
    PointLight(PointLight),
    ParallelLight(ParallelLight),
    GlobalLight(GlobalLight),
}

//
#[derive(Copy,Clone)]
pub struct PointLight{
    pub position: Vec3f,
    pub color: Vec3f,
}

impl PointLight {
    pub fn new(position: Vec3f, color: Vec3f) -> Self {
        PointLight {
            position,
            color,
        }
    }
}

#[derive(Copy, Clone)]
pub struct ParallelLight {
    pub direction: Vec3f,
    pub color: Vec3f,
}

impl ParallelLight {
    pub fn new(direction: Vec3f,color: Vec3f) -> Self {
        Self {
            direction,
            color,
        }
    }
}

#[derive(Copy,Clone)]
pub struct GlobalLight {
    pub color: Vec3f,
}

impl GlobalLight {
    pub fn new(color: Vec3f) -> Self {
        Self {
            color,
        }
    }
}