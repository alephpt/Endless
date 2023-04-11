use crate::graphics::vertex::Vertex;
use crate::graphics::position::Position;
use crate::graphics::normal::Normal;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
        Self { vertices, indices }
    }

    // length of distance between start and end point
    pub fn vertex_length(start: Position, end: Position) -> f32 {
        (end - start).sqrt()
    }

    // direction of the line from start to end point
    pub fn vertex_direction(start: Position, end: Position) -> Position {
        let length = Mesh::vertex_length(start, end);

        end - start / length
    }

    // dot product of two vectors
    pub fn dot_product(a: Position, b: Position) -> f32 {
        a.dot(b)
    }

    // cross product of two vectors
    pub fn cross_product(a: Position, b: Position) -> Position {
        a.cross(b)
    }

    // calculate normal of a triangle surface based on three points
    pub fn normalize(a: Position, b: Position, c: Position) -> Normal {
        (b - a).cross(a - c).into()
    }

    // deduplicate vertices
    pub fn dedup(&mut self) {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        for index in &self.indices {
            let vertex = self.vertices[*index as usize];

            if let Some(i) = vertices.iter().position(|v| *v == vertex) {
                indices.push(i as u16);
            } else {
                indices.push(vertices.len() as u16);
                vertices.push(vertex);
            }
        }

        self.vertices = vertices;
        self.indices = indices;
    }

    // rotate mesh around an axis
    pub fn rotate(&mut self, axis: Position, origin: Position, angle: f32) {
        for vertex in &mut self.vertices {
            vertex.position.rotate(angle, origin,  axis);
        }
    }

    // translate mesh
    pub fn translate(&mut self, position: Position) {
        for vertex in &mut self.vertices {
            vertex.position += position;
        }
    }
}

// add two meshes together
impl std::ops::Add for Mesh {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut vertices = self.vertices;
        let mut indices = self.indices;

        let offset = vertices.len() as u16;

        vertices.extend(other.vertices);
        indices.extend(other.indices.iter().map(|i| i + offset));

        Self { vertices, indices }
    }
}

// implement format
impl std::fmt::Display for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // print each vertex with its index on a new line
        for (i, vertex) in self.vertices.iter().enumerate() {
            writeln!(f, "\t[{}]{}", i, vertex)?;
        }


        // print each index on a new line in groups of 3's
        for (i, index) in self.indices.iter().enumerate() {
            if i % 3 == 0 {
                writeln!(f, "")?;
            }

            write!(f, "\t{} ", index)?;

        }
        Ok(())
    }
}