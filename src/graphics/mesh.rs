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
    pub fn line(start_vertex: Vertex, end_vertex: Vertex,
                     thickness: f32, subdivision: u32) -> Self {
        // define local variables for start and end points and colors
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut start = start_vertex.position;
        let mut end = end_vertex.position;
        let mut start_color = start_vertex.color;
        let mut end_color = end_vertex.color;

        // calculate length, direction, and normal
        let distance = start.distance(end);

        // define half thickness
        let half_thickness = thickness / 2.0;

        // make sure the start is always top left and closer to the camera
        if distance < 0.0 {
            let temp = start;
            let temp_color = start_color;
            start = end;
            start_color = end_color;
            end = temp;
            end_color = temp_color;
        }

        let direction: Position = start.direction(end);
        let normal: Normal = start.cross(end).into();

        // calculate subdivision length, direction, and color
        let subdivision_length = distance / subdivision as f32;
        let subdivision_direction = direction * subdivision_length;
        let subdivision_color = end_color - start_color / subdivision as f32;

        // create vertices
        let mut current = start;
        let mut current_color = start_color;

        // find the first two corners
        let mut c1 = start + normal * half_thickness;
        let mut c2 = start - normal * half_thickness;

        for _ in 0..subdivision {
            // calculate the next point and color
            let next = current + subdivision_direction;
            let next_color = current_color + subdivision_color;

            // calculate the next two corners
            let next_c1 = next + normal * half_thickness;
            let next_c2 = next - normal * half_thickness;

            // calculate the normals
            let normal1: Normal = Mesh::normalize(current, c1, next_c1);
            let normal2: Normal = Mesh::normalize(current, c2, next_c2);

            // add the vertices
            vertices.push(Vertex::new(next_c1, next_color, normal1));
            vertices.push(Vertex::new(next_c2, next_color, normal2));

            // update current
            current = next;
            current_color = next_color;
            c1 = next_c1;
            c2 = next_c2;
        }

        // generate indices based on triangle strip, and connect the end to the start
        for i in 0..subdivision * 2 {
            indices.push(i);
            indices.push(i + 1);
        }         

        // create mesh
        Self{vertices, indices}
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