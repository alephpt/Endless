
use crate::graphics::vertex::Vertex;
use crate::graphics::position::Position;
use crate::graphics::color::Color;
use crate::graphics::normal::Normal;
use crate::graphics::mesh::Mesh;

pub struct Line {
    pub start_position: Position,
    pub end_position: Position,
    pub start_color: Color,
    pub end_color: Color,
    pub thickness: f32,
    pub subdivision: u32,
    pub mesh: Mesh,
}

impl Line {
    // create a line mesh based on a start and end point, thickness, and subdivision rate
    pub fn new(start: Vertex, end: Vertex, thickness: f32, subdivision: u32) -> Self {
        Self {
            start_position: start.position,
            end_position: end.position,
            start_color: start.color,
            end_color: end.color,
            thickness,
            subdivision,
            mesh: Self::line(start, end, thickness, subdivision),
        }
    }

    // calculate length, direction, normal, and create a line mesh based on a thickness and subdivision rate
    pub fn line(start_vertex: Vertex, end_vertex: Vertex, thickness: f32, subdivision: u32) -> Mesh {
        // initialize variables and vectors for mesh
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut current_color = start_vertex.color;
        let mut origin_point = start_vertex.position;
        let color_increment = (end_vertex.color - current_color) / subdivision as f32;
        let half_thickness = thickness / 2.0;
        let direction: Position = end_vertex.position.direction(origin_point);
        let subdivision_increment = direction * origin_point.distance(end_vertex.position) / subdivision as f32;

        // Calculate normal
        let up = if direction.cross(Position::new(0.0, 0.0, 1.0, 1.0)).magnitude() < f32::EPSILON {
            Position::new(0.0, 1.0, 0.0, 1.0)
        } else {
            Position::new(1.0, 0.0, 0.0, 1.0)
        };

        let normal: Normal = if direction.cross(up).magnitude() > f32::EPSILON {
            direction.cross(up).normalize().into()
        } else {
            [1.0, -1.0, 0.0].into()
        };

        // create the first two corners
        let mut p1 = origin_point + normal * half_thickness;
        let mut p2 = origin_point - normal * half_thickness;

        // Create vertices and indices
        for i in 0..=subdivision {
            // add the two corners
            vertices.push(Vertex::new(p1, current_color, normal));
            vertices.push(Vertex::new(p2, current_color, normal));

            // increment the origin point and color
            origin_point += subdivision_increment;
            current_color += color_increment;
            p1 = origin_point + normal * half_thickness;
            p2 = origin_point - normal * half_thickness;

            // add the indices
            if i < subdivision {
                let base = (2 * i) as u16;
                indices.push(base);
                indices.push(base + 1);
                indices.push(base + 2);

                indices.push(base + 1);
                indices.push(base + 3);
                indices.push(base + 2);
            }
        }

        Mesh { vertices, indices }
    }
}

// implement line formatting
impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, 
                "Line:
                    start_position: {}, 
                    end_position: {}, 
                    start_color: {}, 
                    end_color: {}, 
                    thickness: {}, 
                    subdivision: {}, 
                    mesh: \n{}
                ", 
        self.start_position, self.end_position, self.start_color, self.end_color, self.thickness, self.subdivision, self.mesh)
    }
}