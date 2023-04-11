
use crate::graphics::mesh::Mesh;
use crate::graphics::vertex::Vertex;
use crate::graphics::position::Position;
use crate::graphics::color::Color;
use crate::graphics::normal::Normal;

#[derive(Debug)]
pub struct Cube {
    pub fill: bool,
    pub origin: Position,
    pub size: f32,
    pub mesh: Mesh,
}

impl Cube {
    pub fn new(origin: Position, size: f32) -> Self {
        Self {
            fill: true,
            origin,
            size,
            mesh: Cube::cube1(origin, size),
        }
    }

    pub fn cube1(origin: Position, size: f32) -> Mesh {
        let mut vertices = vec![];
        let mut indices = vec![];

        let x = origin.x;
        let y = origin.y;
        let z = origin.z;
        let offset = size / 2.0;


        // create 8 vertices for the cube
        // front face 
        // front right top
        let frt = Vertex::new(
            Position::new(x - offset, y + offset, z + offset, 1.0), 
            Color::black(), 
            Normal::new(0.0, 0.0, 1.0));
        // front left top
        let flt = Vertex::new(
            Position::new(x - offset, y - offset, z + offset, 1.0), 
            Color::cyan(), 
            Normal::new(0.0, 0.0, 1.0));
        // front left bottom
        let flb = Vertex::new(
            Position::new(x + offset, y + offset, z + offset, 1.0), 
            Color::yellow(), 
            Normal::new(0.0, 0.0, 1.0));
        // front right bottom
        let frb = Vertex::new(
            Position::new(x + offset, y - offset, z + offset, 1.0), 
            Color::magenta(), 
            Normal::new(0.0, 0.0, 1.0));

        // back face
        // back left top
        let blt = Vertex::new(
            Position::new(x + offset, y + offset, z - offset, 1.0), 
            Color::green(), 
            Normal::new(0.0, 0.0, 1.0));
        // back right top
        let brt = Vertex::new(
            Position::new(x + offset, y - offset, z - offset, 1.0), 
            Color::blue(), 
            Normal::new(0.0, 0.0, 1.0));
        // back right bottom
        let brb = Vertex::new(
            Position::new(x - offset, y + offset, z - offset, 1.0), 
            Color::red(), 
            Normal::new(0.0, 0.0, 1.0));
        // back left bottom
        let blb = Vertex::new(
            Position::new(x - offset, y - offset, z - offset, 1.0), 
            Color::white(), 
            Normal::new(0.0, 0.0, 1.0));

        // add vertices and indices
        vertices.push(frt);
        vertices.push(flt);
        vertices.push(flb);
        vertices.push(frb);
        vertices.push(blt);
        vertices.push(brt);
        vertices.push(brb);
        vertices.push(blb);

        // wind the indices in clockwise
        // front face         // 012  132
        indices.push(0);
        indices.push(1);
        indices.push(2);

        indices.push(1);
        indices.push(3);
        indices.push(2);

        // back face        // 456 576
        indices.push(4);
        indices.push(5);
        indices.push(6);

        indices.push(5);
        indices.push(7);
        indices.push(6);

        // left face          // 670 710
        indices.push(6);
        indices.push(7);
        indices.push(0);

        indices.push(7);
        indices.push(1);
        indices.push(0);

        // right face        // 234 354
        indices.push(2);
        indices.push(3);
        indices.push(4);

        indices.push(3);
        indices.push(5);
        indices.push(4);

        // top face        // 173 753
        indices.push(1);
        indices.push(7);
        indices.push(3);

        indices.push(7);
        indices.push(5);
        indices.push(4);

        // bottom face     // 604 024
        indices.push(6);
        indices.push(0);
        indices.push(4);

        indices.push(0);
        indices.push(2);
        indices.push(4);

        Mesh::new(vertices, indices)
    }

    // rotate cube around origin
    pub fn rotate(&mut self, angle: f32, axis: Position) {
        self.mesh.rotate(axis, self.origin, angle);
    }

    // translate cube
    pub fn translate(&mut self, displacement: Position) {
        self.origin += displacement;
        self.mesh.translate(displacement);
    }
}

// implement format for the cube
impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Cube {{ 
                        origin: {}, 
                        size: {},
                        Vectors: {{ 
                            vertices: {:?}, 
                            indices: {} }} 
                            ", self.origin, self.size, self.mesh.vertices, self.mesh.indices.len())
    }
}