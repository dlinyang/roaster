use math_utils::Vector3;

pub struct Mesh{
    pub vertex: Vec<Vector3>,
    pub edge: Vec<(usize,usize)>,
    pub face: Vec<Vec<usize>>,
}