
use crate::graphics::mesh::Mesh;
use crate::graphics::vertex::Vertex;
use crate::graphics::position::Position;
use crate::graphics::color::Color;
use crate::graphics::normal::Normal;
use crate::graphics::primitives::quad::Square;


pub struct cube {
    pub fill: bool,
    pub origin: Position,
    pub size: f32,
    pub mesh: Mesh,
}

impl cube {
    pub fn new(origin: Position, size: f32) -> Self {
        Self {
            fill: true,
            origin: origin,
            size: size,
            mesh: cube::cube(origin, size),
        }
    }

    pub fn cube(origin: Position, size: f32) -> Mesh {
        let mut vertices = vec![];
        let mut indices = vec![];

        let x = origin.x;
        let y = origin.y;
        let z = origin.z;
        let center_offset = size / 2.0;

        // create 8 vertices for the cube

        // front face 
        let v1 = Vertex::new(
            Position::new(x - center_offset, y + center_offset, z - center_offset, 1.0), 
            Color::black(), 
            Normal::new(0.0, 0.0, 1.0));
        let v2 = Vertex::new(
            Position::new(x - center_offset, y - center_offset, z - center_offset, 1.0), 
            Color::cyan(), 
            Normal::new(0.0, 0.0, 1.0));
        let v3 = Vertex::new(
            Position::new(x + center_offset, y + center_offset, z - center_offset, 1.0), 
            Color::yellow(), 
            Normal::new(0.0, 0.0, 1.0));
        let v4 = Vertex::new(
            Position::new(x + center_offset, y - center_offset, z - center_offset, 1.0), 
            Color::magenta(), 
            Normal::new(0.0, 0.0, 1.0));

        // back face
        let v5 = Vertex::new(
            Position::new(x - center_offset, y + center_offset, z + center_offset, 1.0), 
            Color::black(), 
            Normal::new(0.0, 0.0, 1.0));
        let v6 = Vertex::new(
            Position::new(x - center_offset, y - center_offset, z + center_offset, 1.0), 
            Color::cyan(), 
            Normal::new(0.0, 0.0, 1.0));
        let v7 = Vertex::new(
            Position::new(x + center_offset, y + center_offset, z + center_offset, 1.0), 
            Color::yellow(), 
            Normal::new(0.0, 0.0, 1.0));
        let v8 = Vertex::new(
            Position::new(x + center_offset, y - center_offset, z + center_offset, 1.0), 
            Color::magenta(), 
            Normal::new(0.0, 0.0, 1.0));

        // create 12 lines and their ccw triangles to make a cube mesh based on a the origin and size
        // front face
        let mut f1 = Square::new(v1, v2, v3, v4);
        // back face
        let mut f2 = Square::new(v5, v6, v7, v8);
        // left face
        let mut f3 = Square::new(v1, v2, v5, v6);
        // right face
        let mut f4 = Square::new(v3, v4, v7, v8);
        // top face
        let mut f5 = Square::new(v1, v3, v5, v7);
        // bottom face
        let mut f6 = Square::new(v2, v4, v6, v8);

        // add the vertices and indices to the mesh
        vertices.append(&mut f1.mesh.vertices);
        vertices.append(&mut f2.mesh.vertices);
        vertices.append(&mut f3.mesh.vertices);
        vertices.append(&mut f4.mesh.vertices);
        vertices.append(&mut f5.mesh.vertices);
        vertices.append(&mut f6.mesh.vertices);

        indices.append(&mut f1.mesh.indices);
        indices.append(&mut f2.mesh.indices);
        indices.append(&mut f3.mesh.indices);
        indices.append(&mut f4.mesh.indices);
        indices.append(&mut f5.mesh.indices);
        indices.append(&mut f6.mesh.indices);

        Mesh::new(vertices, indices)
    }
}