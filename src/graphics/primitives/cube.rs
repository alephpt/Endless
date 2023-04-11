
use crate::graphics::Mesh;
use crate::graphics::Vertex;
use crate::graphics::Position;
use crate::graphics::Color;
use crate::graphics::Normal;
use crate::graphics::Square;

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
            mesh: Cube::cube(origin, size),
        }
    }

    pub fn dedup(&mut self) {
        self.mesh.dedup();
    }

    pub fn cube(origin: Position, size: f32) -> Mesh {
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
        
        // back right bottom
        let brb = Vertex::new(
            Position::new(x + offset, y + offset, z - offset, 1.0), 
            Color::red(), 
            Normal::new(0.0, 0.0, 1.0));
        // back right top
        let brt = Vertex::new(
            Position::new(x + offset, y - offset, z - offset, 1.0), 
            Color::blue(), 
            Normal::new(0.0, 0.0, 1.0));
        // back left bottom
        let blb = Vertex::new(
            Position::new(x - offset, y + offset, z - offset, 1.0), 
            Color::white(), 
            Normal::new(0.0, 0.0, 1.0));
        // back left top
        let blt = Vertex::new(
            Position::new(x - offset, y - offset, z - offset, 1.0), 
            Color::green(), 
            Normal::new(0.0, 0.0, 1.0));




        // add vertices and indices
        vertices.push(frt);             // black - 0
        vertices.push(flt);             // cyan - 1
        vertices.push(flb);             // yellow - 2
        vertices.push(frb);             // magenta - 3
        vertices.push(brb);             // red - 4
        vertices.push(brt);             // blue - 5
        vertices.push(blb);             // white - 6 
        vertices.push(blt);             // green - 7

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
        indices.push(3);

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

    // scale cube
    // pub fn scale(&mut self, scale: f32) {
    //     self.size *= scale;
    //     self.mesh.scale(scale);
    // }

    // subdivide cube surfaces
    pub fn subdivide(&mut self, n_subdivisions: u32) {
        // create new vertex and index vectors
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u16> = vec![];

        // get all 8 corners of the cube
        let flb = self.mesh.vertices[0];
        let flt = self.mesh.vertices[1];
        let frb = self.mesh.vertices[2];
        let frt = self.mesh.vertices[3];
        let brb = self.mesh.vertices[4];
        let brt = self.mesh.vertices[5];
        let blb = self.mesh.vertices[6];
        let blt = self.mesh.vertices[7];
        
        // create a square for each side
        let mut squares: Vec<Square> = vec![];
        squares.push(Square::from_vertices(vec![flb, flt, frb, frt]));
        squares.push(Square::from_vertices(vec![brb, brt, blb, blt]));
        squares.push(Square::from_vertices(vec![blb, blt, flb, flt]));
        squares.push(Square::from_vertices(vec![frb, frt, brb, brt]));
        squares.push(Square::from_vertices(vec![flt, blt, frt, brt]));
        squares.push(Square::from_vertices(vec![blb, flb, brb, frb]));

        // subdivide each square
        for square in &mut squares {
            square.subdivide(n_subdivisions);
        }

        // add vertices and indices to the cube
        for square in squares {
            for index in square.mesh.indices {
                indices.push(vertices.len() as u16 + index);
            }
            for vertex in square.mesh.vertices {
                vertices.push(vertex);
            }
        }

        // update mesh
        self.mesh = Mesh::new(vertices, indices);
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