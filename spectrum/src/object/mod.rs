pub mod sphere;
pub mod mesh;

pub use sphere::Sphere;
pub use mesh::Mesh;

pub enum Object {
    Sphere(Sphere),
    Mesh(Mesh),
}