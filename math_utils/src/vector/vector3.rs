use std::f32;
use num::Float;
use std::ops::{Add,Sub,Mul,Div,Neg,Index,IndexMut};

// a vector in R³ space with 32bit float number
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

//  color  as a Vector alias for  computer graphics
pub type Color = Vector3;
// alias of vector to present point in R³
pub type Point3 = Vector3;

impl Vector3 {
    pub fn new(x: f32, y: f32,z: f32) -> Vector3 {
        Vector3{x: x, y: y, z: z}
    }

    pub fn default() -> Vector3 {
        Vector3{x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn broadcast(a: f32) -> Vector3 {
        Vector3 {x: a, y: a, z: a }
    }

    pub fn length_square(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    /// compute length of R³ Vector
    pub fn length(&self) -> f32 {
        Float::sqrt(self.length_square())
    }
    /// get a normal R³ Vector
    pub fn normalized(&self) -> Vector3 {
        let l = self.length();
        Vector3 {x: self.x / l, y: self.y / l, z: self.z / l }
    }
}
/// dot production for R³
pub fn dot(a: Vector3, b: Vector3) -> f32 {
    a.x * b.x +a.y * b.y + a.z * b.z
}
/// cross production for R³   
pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
    let x = a.y * b.z - a.z * b.y;
    let y = -(a.x * b.z - a.z * b.x);
    let z = a.x * b.y - a.y * b.x;
    Vector3{x: x, y: y, z: z}
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self,rhs: Vector3) -> Vector3 {
        Vector3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self,rhs: Vector3) -> Vector3 {
        Vector3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3{ x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        Vector3{ x: self.x *rhs, y: self.y *rhs, z: self.z * rhs}
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self,rhs: Vector3) -> Vector3 {
        Vector3{x: self * rhs.x, y: self * rhs.y, z: self * rhs.z}
    }
}

impl Div for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Vector3 {
        Vector3 { x: self.x / rhs.x, y: self.y / rhs.y , z: self.z / rhs.z}
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Vector3{
        Vector3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index into R³ vector"),
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i{
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index into R³ vector"),
        }
    }
}