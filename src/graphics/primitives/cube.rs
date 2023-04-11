
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
        vertices.push(frt);             // black - 0
        vertices.push(flt);             // cyan - 1
        vertices.push(flb);             // yellow - 2
        vertices.push(frb);             // magenta - 3
        vertices.push(blt);             // green - 4
        vertices.push(brt);             // blue - 5
        vertices.push(brb);             // red - 6
        vertices.push(blb);             // white - 7 

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
        let frt = self.mesh.vertices[0];
        let flt = self.mesh.vertices[1];
        let flb = self.mesh.vertices[2];
        let frb = self.mesh.vertices[3];
        let blt = self.mesh.vertices[4];
        let brt = self.mesh.vertices[5];
        let brb = self.mesh.vertices[6];
        let blb = self.mesh.vertices[7];
        
        // calculate step size
        let n = 1 << n_subdivisions;
        let step = 1.0 / n as f32;

    // create a new cube based with new tris based on the subdivision size for each face
    for face_index in 0..6 {
        let offset = face_index * n;

        let winding_order = match face_index {
            0 | 2 => [0, 1, 2, 1, 3, 2],
            1 | 3 => [0, 2, 1, 1, 2, 3],
            4 => [0, 1, 3, 1, 2, 3],
            5 => [0, 3, 1, 1, 3, 2],
            _ => unreachable!(),
        };

        let (a, b, c, d) = match face_index {
            0 => (frt, flt, flb, frb), // front
            1 => (blt, brt, brb, blb), // back
            2 => (flt, blt, blb, flb), // left
            3 => (brt, frt, frb, brb), // right
            4 => (blt, flt, frt, brt), // top
            5 => (frb, brb, blb, flb), // bottom
            _ => unreachable!(),
        };

        // create the vertices
        for i in 0..=n {
            let t1 = i as f32 * step;
            for j in 0..=n {
                let t2 = j as f32 * step;
    
                let ab = a.interpolate(b, t2);
                let cd = c.interpolate(d, t2);
                let vert = ab.interpolate(cd, t1);
    
                // add the vertex to the list
                vertices.push(vert);
            }
        }

        // create the indices
        for i in 0..n {
            for j in 0..n {
                let index = i * (n + 1) + j;

                // triangle 1
                indices.push((index + n + 1) as u16);
                indices.push(index as u16);
                indices.push((index + (n + 1) + 1) as u16);

                // triangle 2
                indices.push(index as u16);
                indices.push((index + 1) as u16);
                indices.push((index + (n + 1) + 1) as u16);
            }
        }

    }
    
    // set the new vertices and indices
    self.mesh.vertices = vertices;
    self.mesh.indices = indices;
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