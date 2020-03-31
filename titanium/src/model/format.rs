use crate::base::vertex::Vertex;

pub struct Model {
    pub mesh: Vec<Mesh>,
    pub line: Line,
    pub face: Face,
}

pub type Mesh = Vec<Vertex>;

pub type Line = Vec<(u32,u32)>;

pub type Face = Vec<(u32,u32,u32)>;