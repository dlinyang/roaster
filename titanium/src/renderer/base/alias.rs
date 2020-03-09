//some raw type alias

//2 dimensional vector
pub type Vec2f = [f32;2];

//3 dimensional vector
pub type Vec3f = [f32;3];

//4 dimensional vector
pub type Vec4f = [f32;4];

//3x3 matrix 
pub type Mat3f = [[f32;3];3];

//4x4 matrix
pub type Mat4f = [[f32;4];4];

//4 rank Identity element matrix
pub const Id4f: Mat4f = [[1.0, 0.0, 0.0, 0.0]
                        ,[0.0, 1.0, 0.0, 0.0]
                        ,[0.0, 0.0, 1.0, 0.0]
                        ,[0.0, 0.0, 0.0, 1.0]];