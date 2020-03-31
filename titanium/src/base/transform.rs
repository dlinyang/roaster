use super::alias::{Vec3f, Mat4f, print_mat4};

#[derive(Copy,Clone)]
pub struct Transform {
    pub rotation: Vec3f,
    pub position: Vec3f,
    pub scale: Vec3f,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            rotation: [0.0,0.0,0.0],
            position: [0.0,0.0,0.0],
            scale: [1.0,1.0,1.0],
        }
    }

    pub fn transform(&self) -> Mat4f {
        let position = position(self.position[0], self.position[1], self.position[2]);
        let rotation = rotate(self.rotation[0], self.rotation[1], self.rotation[2]);
        let scale = scale(self.scale[0], self.scale[1], self.scale[2]);

        mat4_x_mat4(mat4_x_mat4(position,rotation), scale)
    }
}

pub fn rotate(x: f32, y: f32,z: f32) -> Mat4f {
    [[z.cos()*x.cos()-y.cos()*x.sin()*z.sin(), -z.sin()*y.sin()*x.sin() - x.cos()*z.sin(), x.sin()*y.sin(),  0.0]
    ,[z.cos()*x.sin()+x.cos()*y.cos()*z.sin(), x.cos()*y.cos()*z.cos() - x.sin()*z.cos(),  -x.cos()*y.sin(), 0.0]
    ,[y.sin()*z.cos(),                         z.sin()*y.cos(),                            y.cos(),       0.0]
    ,[0.0, 0.0, 0.0, 1.0]]
}

pub fn position(x: f32, y: f32, z: f32) -> Mat4f {
    [[1.0, 0.0, 0.0, x]
    ,[0.0, 1.0, 0.0, y]
    ,[0.0, 0.0, 1.0, z]
    ,[0.0, 0.0, 0.0, 1.0]]
}

pub fn scale(x: f32, y: f32, z: f32) -> Mat4f {
    [
        [x  , 0.0, 0.0, 0.0],
        [0.0, y  , 0.0, 0.0],
        [0.0, 0.0, z  , 0.0],
        [0.0, 0.0, 0.0, 1.0]]
}

pub fn perspective(fov: f32, z_far: f32, z_near: f32, aspect_radio: f32) -> Mat4f {
    let f = 1.0/(fov/2.0).tan();
    [
        [f * aspect_radio , 0.0,         0.0         ,          0.0],
        [      0.0        , f  ,         0.0         ,          0.0],
        [      0.0        , 0.0, 2.0/(z_near - z_far), (z_far+z_near)/(z_near - z_far)],
        [      0.0        , 0.0,         0.0         ,          1.0],
    ]
}

pub fn camera(look_from: Vec3f, look_at: Vec3f, vup: Vec3f) -> Mat4f {
 let w = normal(sub(look_from, look_at));
 let u = normal(cross(vup,w));
 let v = cross(w, u);
 let p0 = - look_from[0] * u[0] - look_from[1] * u[1] - look_from[2] * u[2];
 let p1 = - look_from[0] * v[0] - look_from[1] * v[1] - look_from[2] * v[2];
 let p2 = - look_from[0] * w[0] - look_from[1] * w[1] - look_from[2] * w[2];

 [[u[0], v[0], w[0], 0.0]
 ,[u[1], v[1], w[1], 0.0]
 ,[u[2], v[2], w[2], 0.0]
 ,[p0  , p1  , p2  ,  1.0]]
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

pub fn mat4_x_mat4(a: Mat4f, b: Mat4f) -> Mat4f {
    let c00 = a[0][0] * b[0][0] + a[0][1] * b[1][0] + a[0][2] * b[2][0] + a[0][3] * b[3][0];
    let c01 = a[0][0] * b[0][1] + a[0][1] * b[1][1] + a[0][2] * b[2][1] + a[0][3] * b[3][1];
    let c02 = a[0][0] * b[0][2] + a[0][1] * b[1][2] + a[0][2] * b[2][2] + a[0][3] * b[3][2];
    let c03 = a[0][0] * b[0][3] + a[0][1] * b[1][3] + a[0][2] * b[2][3] + a[0][3] * b[3][3];
    //
    let c10 = a[1][0] * b[0][0] + a[1][1] * b[1][0] + a[1][2] * b[2][0] + a[1][3] * b[3][0];
    let c11 = a[1][0] * b[0][1] + a[1][1] * b[1][1] + a[1][2] * b[2][1] + a[1][3] * b[3][1];
    let c12 = a[1][0] * b[0][2] + a[1][1] * b[1][2] + a[1][2] * b[2][2] + a[1][3] * b[3][2];
    let c13 = a[1][0] * b[0][3] + a[1][1] * b[1][3] + a[1][2] * b[2][3] + a[1][3] * b[3][3];
    //
    let c20 = a[2][0] * b[0][0] + a[2][1] * b[1][0] + a[2][2] * b[2][0] + a[2][3] * b[3][0];
    let c21 = a[2][0] * b[0][1] + a[2][1] * b[1][1] + a[2][2] * b[2][1] + a[2][3] * b[3][1];
    let c22 = a[2][0] * b[0][2] + a[2][1] * b[1][2] + a[2][2] * b[2][2] + a[2][3] * b[3][2];
    let c23 = a[2][0] * b[0][3] + a[2][1] * b[1][3] + a[2][2] * b[2][3] + a[2][3] * b[3][3];
    //
    let c30 = a[3][0] * b[0][0] + a[3][1] * b[1][0] + a[3][2] * b[2][0] + a[3][3] * b[3][0];
    let c31 = a[3][0] * b[0][1] + a[3][1] * b[1][1] + a[3][2] * b[2][1] + a[3][3] * b[3][1];
    let c32 = a[3][0] * b[0][2] + a[3][1] * b[1][2] + a[3][2] * b[2][2] + a[3][3] * b[3][2];
    let c33 = a[3][0] * b[0][3] + a[3][1] * b[1][3] + a[3][2] * b[2][3] + a[3][3] * b[3][3];

    [[c00, c01, c02, c03],
     [c10, c11, c12, c13],
     [c20, c21, c22, c23],
     [c30, c31, c32, c33]]
}