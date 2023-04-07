use crate::graphics::vertex::Vertex;
use crate::graphics::position::Position;
use crate::graphics::normal::Normal;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    // length of distance between start and end point
    pub fn vertex_length(start: [f32; 4], end: [f32; 4]) -> f32 {
        let x = end[0] - start[0];
        let y = end[1] - start[1];
        let z = end[2] - start[2];

        (x * x + y * y + z * z).sqrt()
    }

    // direction of the line from start to end point
    pub fn vertex_direction(start: [f32; 4], end: [f32; 4]) -> [f32; 4] {
        let length = Mesh::vertex_length(start, end);

        let x = (end[0] - start[0]) / length;
        let y = (end[1] - start[1]) / length;
        let z = (end[2] - start[2]) / length;

        [x, y, z, 0.0]
    }

    // dot product of two vectors
    pub fn dot_product(a: [f32; 4], b: [f32; 4]) -> f32 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    // cross product of two vectors
    pub fn cross_product(a: [f32; 4], b: [f32; 4]) -> [f32; 3] {
        let x = a[1] * b[2] - a[2] * b[1];
        let y = a[2] * b[0] - a[0] * b[2];
        let z = a[0] * b[1] - a[1] * b[0];

        [x, y, z]
    }

    // calculate normal of a triangle surface based on three points
    pub fn normalize(a: Position, b: Position, c: Position) -> Normal {
        (b - a).cross(a - c).into()
    }
}

// implement format
impl std::fmt::Display for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // print each vertex on a new line
        for vertex in &self.vertices {
            writeln!(f, "{}", vertex)?;
        }

        // print each index on a new line in groups of 3's
        for (i, index) in self.indices.iter().enumerate() {
            if i % 3 == 0 {
                writeln!(f, "")?;
            }

            write!(f, "{} ", index)?;

        }
        Ok(())
    }
}