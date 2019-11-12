use super::alias::{Vec3f, Mat4f};

pub fn rotate(x: f32, y: f32,z: f32) -> Mat4f {
    [[z.cos()*x.cos()-y.cos()*x.sin()*z.sin(), -z.sin()*y.sin()*x.sin() - x.cos()*z.sin(), x.sin()*y.sin(),  0.0]
    ,[z.cos()*x.sin()+x.cos()*y.cos()*z.sin(), x.cos()*y.cos()*z.cos() - x.sin()*z.cos(),  -x.cos()*y.sin(), 0.0]
    ,[y.sin()*z.cos(),                         z.cos()*y.cos(),                            y.cos(),       0.0]
    ,[0.0, 0.0, 0.0, 1.0]]
}

pub fn position(x: f32, y: f32, z: f32) -> Mat4f {
    [[1.0, 0.0, 0.0, x]
    ,[0.0, 1.0, 0.0, y]
    ,[0.0, 0.0, 1.0, z]
    ,[0.0, 0.0, 0.0, 1.0]]
}

pub fn camera(look_from: Vec3f, look_at: Vec3f, vup: Vec3f) -> Mat4f {
 let w = normal(sub(look_from, look_at));
 let u = normal(cross(vup,w));
 let v = cross(w, u);
 [[w[0], w[1], w[2], -w[0]*look_from[0]-w[1]*look_from[1]-w[2]*look_from[2]]
 ,[v[0], v[1], v[2], -v[0]*look_from[0]-v[1]*look_from[1]-v[2]*look_from[2]]
 ,[u[0], u[1], u[2], -u[0]*look_from[0]-u[1]*look_from[1]-u[2]*look_from[2]]
 ,[0.0,  0.0,  0.0,  1.0]]
}

#[inline]
fn normal(a: Vec3f) -> Vec3f {
    let len = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
    [a[0]/len, a[1]/len, a[2]/len]
}

#[inline]
fn sub(a: Vec3f, b: Vec3f) -> Vec3f {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
fn cross(a: Vec3f, b: Vec3f) -> Vec3f {
    [a[1]*b[2] - a[2]*b[1]
    ,a[2]*b[0] - a[0]*b[2]
    ,a[0]*b[1] - a[1]*b[0]]
}