use crate::graphics::Mesh;
use crate::graphics::Vertex;

pub struct Square {
    pub fill: bool, // used to determine if these are lines of solid squares
    pub mesh: Mesh,
}

impl Square {
    // instantiate a new square
    pub fn new(v1: Vertex, v2: Vertex, v3: Vertex, v4: Vertex) -> Self {
        Self {
            fill: true,
            mesh: Square::square(v1, v2, v3, v4),
        }
    }

    // create a square mesh based on four vertices
    pub fn square(v1: Vertex, v2: Vertex, v3: Vertex, v4: Vertex) -> Mesh {
        Mesh::new(vec![v1, v2, v3, v4], vec![0, 1, 2, 1, 3, 2])
    }
}