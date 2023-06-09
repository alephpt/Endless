use crate::graphics::Mesh;
use crate::graphics::Vertex;
use crate::graphics::Position;
use crate::graphics::Color;
use crate::graphics::Normal;
use crate::graphics::Geometry;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub fill: bool, // used to determine if these are lines of solid triangles
    pub origin: Position,
    pub size: f32,
    pub mesh: Mesh,
}

impl Triangle {
    pub fn new(position: Position, size: f32) -> Geometry {
        Geometry::Triangle(Self::triangle(position, size))
    }

    // instantiate a new triangle
    pub fn triangle(origin: Position, size: f32) -> Self {
        Self {
            fill: true,
            origin, 
            size,
            mesh: Triangle::mesh(origin, size),
        }
    }

    // create a triangle mesh based on three vertices
    pub fn mesh(origin: Position, size: f32) -> Mesh {
        // calculate the altitude of the triangle
        let altitude = (size * 0.5).sqrt();

        // calculate the offsets of the triangle
        let offset_a = [0.0, altitude * 0.75];
        let offset_b = [size * 0.5 * -1.0, -altitude / 2.0];
        let offset_c = [size * 0.5, -altitude / 2.0];

        // create the vertices of the triangle
        let vertices = vec![
            Vertex::new(
                Position::new(origin.x + offset_a[0], origin.y + offset_a[1], origin.z, 1.0),
                Color::new(1.0, 0.0, 0.0, 1.0),
                Normal::new(0.0, 0.0, 1.0),
            ),
            Vertex::new(
                Position::new(origin.x + offset_b[0], origin.y + offset_b[1], origin.z, 1.0),
                Color::new(0.0, 1.0, 0.0, 1.0),
                Normal::new(0.0, 0.0, 1.0),
            ),
            Vertex::new(
                Position::new(origin.x + offset_c[0], origin.y + offset_c[1], origin.z, 1.0),
                Color::new(0.0, 0.0, 1.0, 1.0),
                Normal::new(0.0, 0.0, 1.0),
            ),
        ];
        // create a mesh from the vertices and indices
        Mesh::new(vertices, vec![0, 1, 2])
    }

    // subdivide the triangle
    pub fn subdivide(&mut self, _iterations: u32) {
        return;
    }
}