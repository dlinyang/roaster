//some raw type alias

//
pub type Vec2f = [f32;2];
//
pub type Vec3f = [f32;3];
//
pub type Vec4f = [f32;4];
//
pub type Mat3f = [[f32;3];3];
//
pub type Mat4f = [[f32;4];4];

pub const Id4f: Mat4f = [[1.0, 0.0, 0.0, 0.0]
                        ,[0.0, 1.0, 0.0, 0.0]
                        ,[0.0, 0.0, 1.0, 0.0]
                        ,[0.0, 0.0, 0.0, 1.0]];