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

    pub fn dedup(&self) -> Self {
        let mut vertices = vec![];
        let mut indices = vec![];

        for index in self.mesh.indices.iter() {
            let vertex = self.mesh.vertices[*index as usize];
            let mut found = false;
            for (i, v) in vertices.iter().enumerate() {
                if v == &vertex {
                    indices.push(i as u16);
                    found = true;
                    break;
                }
            }
            if !found {
                indices.push(vertices.len() as u16);
                vertices.push(vertex);
            }
        }

        Self {
            fill: self.fill,
            mesh: Mesh::new(vertices, indices),
        }
    }

    // subdivide a square based on a subdivision rate
    pub fn subdivide(&self, n_subdivisions: u32) -> Self {
        let mut vertices = vec![];
        let mut indices = vec![];

        let v1 = self.mesh.vertices[0];
        let v2 = self.mesh.vertices[1];
        let v3 = self.mesh.vertices[2];
        let v4 = self.mesh.vertices[3];
        let n = 1 << n_subdivisions;
        let step = 1.0 / n as f32;
    
        for i in 0..=n {
            let t1 = i as f32 * step;
            for j in 0..=n {
                let t2 = j as f32 * step;
    
                let position = v1.position * (1.0 - t1) * (1.0 - t2)
                    + v2.position * (1.0 - t1) * t2
                    + v3.position * t1 * (1.0 - t2)
                    + v4.position * t1 * t2;
    
                let color = v1.color * (1.0 - t1) * (1.0 - t2)
                    + v2.color * (1.0 - t1) * t2
                    + v3.color * t1 * (1.0 - t2)
                    + v4.color * t1 * t2;
    
                let normal = v1.normal * (1.0 - t1) * (1.0 - t2)
                    + v2.normal * (1.0 - t1) * t2
                    + v3.normal * t1 * (1.0 - t2)
                    + v4.normal * t1 * t2;
    
                vertices.push(Vertex::new(position, color, normal));
            }
        }
    
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

        Self {
            fill: self.fill,
            mesh: Mesh::new(vertices, indices),
        }.dedup()
    }
}
