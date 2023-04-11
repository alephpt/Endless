use crate::graphics::Mesh;
use crate::graphics::Vertex;
use crate::graphics::Position;
use crate::graphics::Color;
use crate::graphics::Normal;

pub struct Square {
    pub fill: bool, // used to determine if these are lines of solid squares
    pub mesh: Mesh,
}

impl Square {
    // instantiate a new square
    pub fn new(origin: Position, size: f32) -> Self {
        Self {
            fill: true,
            mesh: Square::square(origin, size),
        }
    }

    // create a square mesh based on four vertices
    pub fn square(origin: Position, size: f32) -> Mesh {
        let x = origin.x;
        let y = origin.y;
        let z = origin.z;

        let offset = size / 2.0;

        let v1 = Vertex::new(
            Position::new(x - offset, y + offset, z, 1.0), 
            Color::black(), 
            Normal::new(0.0, 0.0, 1.0),
        );

        let v2 = Vertex::new(
            Position::new(x - offset, y - offset, 0.0, 1.0), 
            Color::cyan(), 
            Normal::new(0.0, 0.0, 1.0),
        );

        let v3 = Vertex::new(
            Position::new(x + offset, y + offset, 0.0, 1.0), 
            Color::yellow(), 
            Normal::new(0.0, 0.0, 1.0)
        );

        let v4 = Vertex::new(
            Position::new(x + offset, y - offset, 0.0, 1.0), 
            Color::magenta(), 
            Normal::new(0.0, 0.0, 1.0)
        );

        Mesh::new(vec![v1, v2, v3, v4], vec![0, 1, 2, 1, 3, 2])
    }

    pub fn dedup(&mut self) {
        self.mesh.dedup();
    }

    // subdivide a square based on a subdivision rate
    pub fn subdivide(&mut self, n_subdivisions: u32) {
        let mut vertices = vec![];
        let mut indices = vec![];

        // get the four vertices of the square
        let v1 = self.mesh.vertices[0];
        let v2 = self.mesh.vertices[1];
        let v3 = self.mesh.vertices[2];
        let v4 = self.mesh.vertices[3];

        // calculate the number of subdivisions
        let n = 1 << n_subdivisions;
        // calculate the step size
        let step = 1.0 / n as f32;
    
        // create the vertices
        for i in 0..=n {
            let t1 = i as f32 * step;
            for j in 0..=n {
                let t2 = j as f32 * step;
    
                // calculate the position of the vertex
                let position = v1.position * (1.0 - t1) * (1.0 - t2)
                    + v2.position * (1.0 - t1) * t2
                    + v3.position * t1 * (1.0 - t2)
                    + v4.position * t1 * t2;
    
                // calculate the color of the vertex
                let color = v1.color * (1.0 - t1) * (1.0 - t2)
                    + v2.color * (1.0 - t1) * t2
                    + v3.color * t1 * (1.0 - t2)
                    + v4.color * t1 * t2;
    
                // calculate the normal of the vertex
                let normal = v1.normal * (1.0 - t1) * (1.0 - t2)
                    + v2.normal * (1.0 - t1) * t2
                    + v3.normal * t1 * (1.0 - t2)
                    + v4.normal * t1 * t2;
    
                // add the vertex to the list
                vertices.push(Vertex::new(position, color, normal));
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

        // create the mesh
        self.mesh = Mesh::new(vertices, indices);
        self.dedup();
    }
}
