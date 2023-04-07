use crate::graphics::mesh::Mesh;
use crate::graphics::vertex::Vertex;

pub struct Triangle {
    pub fill: bool, // used to determine if these are lines of solid triangles
    pub mesh: Mesh,
}

impl Triangle {
    // instantiate a new triangle
    pub fn new(v1: Vertex, v2: Vertex, v3: Vertex) -> Self {
        Self {
            fill: true,
            mesh: Triangle::triangle(v1, v2, v3),
        }
    }

    // create a triangle mesh based on three vertices
    pub fn triangle(v1: Vertex, v2: Vertex, v3: Vertex) -> Mesh {
        Mesh::new(vec![v1, v2, v3], vec![0, 1, 2])
    }
}