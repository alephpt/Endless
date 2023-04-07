    use crate::graphics::vertex::Vertex;
    use crate::graphics::position::Position;
    use crate::graphics::color::{Color, self};
    use crate::graphics::mesh::Mesh;

    pub struct Ring {
        pub center: Position,
        pub radius: f32,
        pub thickness: f32,
        pub subdivision: u32,
        pub color: Color,
        pub mesh: Mesh,
    }
        
    impl Ring {
        // instantiate a new ring
        pub fn new(center: Position, radius: f32, thickness: f32, subdivision: u32, color: Color) -> Self {
            Self {
                center,
                radius,
                thickness,
                subdivision,
                color,
                mesh: Ring::ring(center, radius, thickness, subdivision, color)
            }
        }


        // create a circular ring mesh based on a center point, radius, color, line thickness, and subdivision rate
        pub fn ring(center: Position, radius: f32, thickness: f32, subdivision: u32, color: Color) -> Mesh {
            // create vertices, angles, color, and normal
            let mut vertices = Vec::new();
            let mut indices = Vec::new();
            let angle = 2.0 * std::f32::consts::PI / subdivision as f32;
            let axis = [0.0, 0.0, 1.0, 0.0].into();
            let half_thickness = thickness / 2.0;

            // create the first two corners
            let mut p1 = Position::new(center[0] + radius + half_thickness, center[1], center[2], 1.0);
            let mut p2 = Position::new(center[0] + radius - half_thickness, center[1], center[2], 1.0);

            // create the first two normals
            let mut normal1 = Mesh::normalize(center, p1, p2);
            let mut normal2 = Mesh::normalize(center, p2, p1);

            
            // create the first two vertices
            vertices.push(Vertex::new(p1, color, normal1));
            vertices.push(Vertex::new(p2, color, normal2));

            // create the first two indices
            indices.push(0);
            indices.push(1);

            // create the rest of the vertices and indices
            for i in 1..subdivision {
                // calculate the next two corners
                let next_p1 = p1.rotate(angle, center, axis);
                let next_p2 = p2.rotate(angle, center, axis);

                // calculate the next two normals
                normal1 =  Mesh::normalize(p1, next_p1, next_p2);
                normal2 =  Mesh::normalize(p2, next_p2, next_p1);

                // add the vertices
                vertices.push(Vertex::new(next_p1, color, normal1));
                vertices.push(Vertex::new(next_p2, color, normal2));

                // add indices in triangle strip order
                indices.push(i * 2);
                indices.push(i * 2 + 1);
                

                // update p1 and p2
                p1 = next_p1;
                p2 = next_p2;
            }

            // update original vertices normals 
            vertices[0].normal = Mesh::normalize(vertices[vertices.len() - 2].position, vertices[0].position, vertices[1].position);
            vertices[1].normal = Mesh::normalize(vertices[vertices.len() - 1].position, vertices[1].position, vertices[0].position);

            // create mesh
            Mesh{vertices, indices}
        }
    }