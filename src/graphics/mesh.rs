use crate::graphics::vertex::Vertex;
use crate::graphics::position::Position;
use crate::graphics::normal::Normal;
use crate::graphics::color::Color;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    // length of distance between start and end point
    pub fn vertex_length(start: [f32; 4], end: [f32; 4]) -> f32 {
        let x = end[0] - start[0];
        let y = end[1] - start[1];
        let z = end[2] - start[2];

        (x * x + y * y + z * z).sqrt()
    }

    // direction of the line from start to end point
    pub fn vertex_direction(start: [f32; 4], end: [f32; 4]) -> [f32; 4] {
        let length = Mesh::vertex_length(start, end);

        let x = (end[0] - start[0]) / length;
        let y = (end[1] - start[1]) / length;
        let z = (end[2] - start[2]) / length;

        [x, y, z, 0.0]
    }

    // dot product of two vectors
    pub fn dot_product(a: [f32; 4], b: [f32; 4]) -> f32 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    // cross product of two vectors
    pub fn cross_product(a: [f32; 4], b: [f32; 4]) -> [f32; 3] {
        let x = a[1] * b[2] - a[2] * b[1];
        let y = a[2] * b[0] - a[0] * b[2];
        let z = a[0] * b[1] - a[1] * b[0];

        [x, y, z]
    }

    // calculate normal of a triangle surface based on three points
    pub fn normalize(a: Position, b: Position, c: Position) -> Normal {
        (b - a).cross(a - c).into()
    }

    // calculate length, direction, normal, and create a line mesh based on a thickness and subdivision rate
    pub fn line(start_vertex: Vertex, end_vertex: Vertex, thickness: f32, subdivision: u32) -> Self {
        // initialize variables and vectors for mesh
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut current_color = start_vertex.color;
        let mut origin_point = start_vertex.position;
        let color_incriment = (end_vertex.color - current_color) / subdivision as f32;
        let half_thickness = thickness / 2.0;
        let direction: Position = origin_point.direction(end_vertex.position);
        let subdivision_incriment = direction * origin_point.distance(end_vertex.position) / subdivision as f32;

        // Calculate normal
        let up = if direction.cross(Position::new(0.0, 1.0, 0.0, 1.0)).magnitude() < f32::EPSILON {
            Position::new(0.0, 0.0, 1.0, 1.0)
        } else {
            Position::new(0.0, 1.0, 0.0, 1.0)
        };

        let normal: Normal = if direction.cross(up).magnitude() > f32::EPSILON {
            direction.cross(up).normalize().into()
        } else {
            [1.0, 0.0, 0.0].into()
        };

        // create the first two corners
        let mut p1 = origin_point + normal * half_thickness;
        let mut p2 = origin_point - normal * half_thickness;

        // create the first two vertices
        vertices.push(Vertex::new(p1, current_color, normal));
        vertices.push(Vertex::new(p2, current_color, normal));

        origin_point = (p1 + p2) / 2.0;

        // create the rest of the vertices
        for i in 0..subdivision {
            let base = 2 * i;
            origin_point += subdivision_incriment;
            current_color += color_incriment;
            p1 = origin_point + normal * half_thickness;
            p2 = origin_point - normal * half_thickness;
            origin_point = (p1 + p2) / 2.0;

            vertices.push(Vertex::new(p1, current_color, normal));
            vertices.push(Vertex::new(p2, current_color, normal));

            indices.push(base);
            indices.push(base + 1);
            indices.push(base + 2);
        
            indices.push(base + 1);
            indices.push(base + 3);
            indices.push(base + 2);
        }

        Self { vertices, indices }
    }

    // create a circular ring mesh based on a center point, radius, color, line thickness, and subdivision rate
    pub fn ring(center: Position, radius: f32, thickness: f32, subdivision: u32, color: Color) -> Self {
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
        Self{vertices, indices}
    }
}
