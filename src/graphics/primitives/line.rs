
use crate::graphics::vertex::Vertex;
use crate::graphics::position::Position;
use crate::graphics::color::Color;
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
    pub fn line(start_vertex: Vertex, end_vertex: Vertex, _thickness: f32, _subdivision: u32) -> Mesh {
        // determine the length of the mesh
        let _length = Mesh::vertex_length(start_vertex.position.into(), end_vertex.position.into());

        // return new mesh
        Mesh::new(
            // create vertices
             vec![start_vertex, end_vertex],
            // create indices
         vec![0, 1]
        )
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