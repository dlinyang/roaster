/// The raw math type alias

///2 dimensional vector
pub type Vec2f = [f32;2];

///3 dimensional vector
pub type Vec3f = [f32;3];

/// 4 dimensional vector
pub type Vec4f = [f32;4];

/// 2x2 matrix
pub type Mat2f = [[f32;2];2];

/// 3x3 matrix 
pub type Mat3f = [[f32;3];3];

/// 4x4 matrix
pub type Mat4f = [[f32;4];4];

/// 4 rank Identity element matrix
pub const Id4f: Mat4f = [[1.0, 0.0, 0.0, 0.0]
                        ,[0.0, 1.0, 0.0, 0.0]
                        ,[0.0, 0.0, 1.0, 0.0]
                        ,[0.0, 0.0, 0.0, 1.0]];

pub fn rgba_to_rgb(color: Vec4f) -> Vec3f {
    [color[0], color[1], color[2]]
}

pub fn print_mat4(mat: Mat4f) {
    let mut i = 0;                            
    let mut j = 0;
    while i < 4 {
        while j < 4 {
            print!("{} ",mat[i][j]);
            j = j + 1;
        }
        println!("");
        j = 0;
        i = i+ 1;
    }
}