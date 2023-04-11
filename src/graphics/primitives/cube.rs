
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
    pub fn subdivide(&mut self, subdivisions: u32) {
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

        // create a new cube based with new tris based on the subdivision size for each face
        for i in 0..subdivisions {
            for j in 0..subdivisions {
                let t1 = i as f32 / subdivisions as f32;
                let t2 = (i + 1) as f32 / subdivisions as f32;
                let s1 = j as f32 / subdivisions as f32;
                let s2 = (j + 1) as f32 / subdivisions as f32;

                // calculate new verts for each face
                for &(a, b, c, d) in &[
                    (frt, flt, flb, frb), // front
                    (blt, brt, brb, blb), // back
                    (flt, blt, blb, flb), // left
                    (brt, frt, frb, brb), // right
                    (flt, blt, brt, frt), // top
                    (flb, blb, brb, frb), // bottom
                ] {
                    let ac_t1 = a.interpolate(c, t1);
                    let ac_t2 = a.interpolate(c, t2);
                    let bd_t1 = b.interpolate(d, t1);
                    let bd_t2 = b.interpolate(d, t2);
    
                    let v1 = ac_t1.interpolate(bd_t1, s1);
                    let v2 = ac_t1.interpolate(bd_t1, s2);
                    let v3 = ac_t2.interpolate(bd_t2, s1);
                    let v4 = ac_t2.interpolate(bd_t2, s2);
    
                    // calculate the index
                    let index = vertices.len() as u16;
    
                    // add new vertices
                    vertices.push(v1);
                    vertices.push(v2);
                    vertices.push(v3);
                    vertices.push(v4);
    
                    // add new indices
                    indices.push(index + 2);
                    indices.push(index + 3);
                    indices.push(index + 1);
                    indices.push(index + 2);
                    indices.push(index + 1);
                    indices.push(index);
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