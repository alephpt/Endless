


pub struct Color(pub [f32; 4]);

// define a series of primary color 
// Color::black(), Color::white(), Color::red(), Color::green(), Color::blue(), Color::yellow()
impl Color {
    pub fn black() -> Self {
        Self([0.0, 0.0, 0.0, 1.0])
    }

    pub fn white() -> Self {
        Self([1.0, 1.0, 1.0, 1.0])
    }

    pub fn red() -> Self {
        Self([1.0, 0.0, 0.0, 1.0])
    }

    pub fn green() -> Self {
        Self([0.0, 1.0, 0.0, 1.0])
    }

    pub fn blue() -> Self {
        Self([0.0, 0.0, 1.0, 1.0])
    }

    pub fn yellow() -> Self {
        Self([1.0, 1.0, 0.0, 1.0])
    }

    pub fn cyan() -> Self {
        Self([0.0, 1.0, 1.0, 1.0])
    }

    pub fn magenta() -> Self {
        Self([1.0, 0.0, 1.0, 1.0])
    }

    pub fn gray() -> Self {
        Self([0.5, 0.5, 0.5, 1.0])
    }

    pub fn orange() -> Self {
        Self([1.0, 0.5, 0.0, 1.0])
    }

    pub fn purple() -> Self {
        Self([0.5, 0.0, 0.5, 1.0])
    }

    pub fn brown() -> Self {
        Self([0.5, 0.25, 0.0, 1.0])
    }

    pub fn pink() -> Self {
        Self([1.0, 0.75, 0.8, 1.0])
    }

    pub fn lime() -> Self {
        Self([0.75, 1.0, 0.0, 1.0])
    }

    pub fn turquoise() -> Self {
        Self([0.25, 0.88, 0.82, 1.0])
    }

    pub fn silver() -> Self {
        Self([0.75, 0.75, 0.75, 1.0])
    }

    pub fn gold() -> Self {
        Self([1.0, 0.84, 0.0, 1.0])
    }

    pub fn teal() -> Self {
        Self([0.0, 0.5, 0.5, 1.0])
    }

    pub fn navy() -> Self {
        Self([0.0, 0.0, 0.5, 1.0])
    }

    pub fn maroon() -> Self {
        Self([0.5, 0.0, 0.0, 1.0])
    }

    pub fn olive() -> Self {
        Self([0.5, 0.5, 0.0, 1.0])
    }

    pub fn random() -> Self {
        Self([rand::random(), rand::random(), rand::random(), 1.0])
    }

    pub fn add(&mut self, color: [f32; 4]) {
        self.0[0] += color[0];
        self.0[1] += color[1];
        self.0[2] += color[2];
        self.0[3] += color[3];
    }

    pub fn sub(&mut self, color: [f32; 4]) {
        self.0[0] -= color[0];
        self.0[1] -= color[1];
        self.0[2] -= color[2];
        self.0[3] -= color[3];
    }

    pub fn mul(&mut self, color: [f32; 4]) {
        self.0[0] *= color[0];
        self.0[1] *= color[1];
        self.0[2] *= color[2];
        self.0[3] *= color[3];
    }

    pub fn div(&mut self, color: [f32; 4]) {
        self.0[0] /= color[0];
        self.0[1] /= color[1];
        self.0[2] /= color[2];
        self.0[3] /= color[3];
    }
}



#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
    pub color: Color,
}

impl Vertex {
    pub fn new(position: [f32; 4], color: [f32; 4]) -> Self {
        Self { position, color }
    }

    pub fn setColor(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    pub fn updatePosition(&mut self, position: [f32; 4]) {
        self.position = position;
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4
                },
            ]
        }
    }
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
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
    pub fn cross_product(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
        let x = a[1] * b[2] - a[2] * b[1];
        let y = a[2] * b[0] - a[0] * b[2];
        let z = a[0] * b[1] - a[1] * b[0];

        [x, y, z, 0.0]
    }

    // calculate normal of a triangle
    pub fn triangle_normal(a: [f32; 4], b: [f32; 4], c: [f32; 4]) -> [f32; 4] {
        let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2], 0.0];
        let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2], 0.0];

        Mesh::cross_product(ab, ac)
    }

    // calculate length, direction, normal, and create a line mesh based on a thickness and subdivision rate
    pub fn line_mesh(start_vertex: Vertex, end_vertex: Vertex,
                     thickness: f32, subdivision: u32) -> Self {
        // define local variables for start and end points and colors
        let start = start_vertex.position;
        let end = end_vertex.position;
        let start_color = start_vertex.color;
        let end_color = end_vertex.color;

        // calculate length, direction, and normal
        let length = Mesh::vertex_length(start, end);
        let direction = Mesh::vertex_direction(start, end);
        let normal = Mesh::cross_product(direction, [0.0, 0.0, 1.0, 0.0]);

        // define half thickness
        let half_thickness = thickness / 2.0;

        // make sure the normal is pointing in the right direction
        if normal[2] < 0.0 {
            normal[0] *= -1.0;
            normal[1] *= -1.0;
            normal[2] *= -1.0;
        }

        // make sure the start is always top left and closer to the camera
        if Mesh::direction(start, end) < 0.0 {
            let temp = start;
            temp_color = start_color;
            start = end;
            start_color = end_color;
            end = temp;
            end_color = temp_color;
        }

        // find the four corners of the line
        let c1 = [start[0] + normal[0] * half_thickness, 
                  start[1] + normal[1] * half_thickness, 
                  start[2] + normal[2] * half_thickness, 0.0];
        let c2 = [start[0] - normal[0] * half_thickness,
                  start[1] - normal[1] * half_thickness,
                  start[2] - normal[2] * half_thickness, 0.0];
        let c3 = [end[0] + normal[0] * half_thickness,
                  end[1] + normal[1] * half_thickness,
                  end[2] + normal[2] * half_thickness, 0.0];
        let c4 = [end[0] - normal[0] * half_thickness,
                  end[1] - normal[1] * half_thickness,
                  end[2] - normal[2] * half_thickness, 0.0];
                

        // calculate subdivision length and direction
        let subdivision_length = length / subdivision as f32;
        let subdivision_direction = [direction[0] * subdivision_length, 
                                     direction[1] * subdivision_length, 
                                     direction[2] * subdivision_length, 0.0];

        // calculate subdivision color
        let subdivision_color = [(end_color[0] - start_color[0]) / subdivision as f32,
                                 (end_color[1] - start_color[1]) / subdivision as f32,
                                 (end_color[2] - start_color[2]) / subdivision as f32];

        // create vertices
        let mut vertices = Vec::new();
        let mut current = start;
        let mut current_color = start_color;

        for _ in 0..subdivision {
            // calculate the next point
            let next = [current[0] + subdivision_direction[0],
                        current[1] + subdivision_direction[1],
                        current[2] + subdivision_direction[2], 0.0];

            // calculate the next color
            let next_color = [current_color[0] + subdivision_color[0],
                              current_color[1] + subdivision_color[1],
                              current_color[2] + subdivision_color[2], 0.0];

            // calculate the next two corners
            let next_c1 = [next[0] + normal[0] * half_thickness,
                           next[1] + normal[1] * half_thickness,
                           next[2] + normal[2] * half_thickness, 0.0];
            let next_c2 = [next[0] - normal[0] * half_thickness,
                           next[1] - normal[1] * half_thickness,
                           next[2] - normal[2] * half_thickness, 0.0];

            // calculate the normals
            let normal1 = Mesh::triangle_normal(current, next_c1, c1);
            let normal2 = Mesh::triangle_normal(current, next_c2, c2);
            let normal3 = Mesh::triangle_normal(next, next_c1, c3);
            let normal4 = Mesh::triangle_normal(next, next_c2, c4);

            // add the vertices
            vertices.push(Vertex::new(current, current_color, normal1));
            vertices.push(Vertex::new(current, current_color, normal2));
            vertices.push(Vertex::new(next, next_color, normal3));
            vertices.push(Vertex::new(next, next_color, normal4));

            // update current
            current = next;
            current_color = next_color;
        }

        // create indices
        let mut indices = Vec::new();
        for i in 0..subdivision {
            indices.push(i * 4);
            indices.push(i * 4 + 1);
            indices.push(i * 4 + 2);
            indices.push(i * 4 + 1);
            indices.push(i * 4 + 3);
            indices.push(i * 4 + 2);
        }

        // create mesh
        Self{vertices, indices}
    }

    // create a circular ring mesh based on a center point, radius, color, line thickness, and subdivision rate
    pub fn ring_mesh(center: [f32; 4], radius: f32, color: [f32; 4], thickness: f32, subdivision: u32) -> Self {
        // create vertices
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // calculate subdivision angle
        let subdivision_angle = 2.0 * std::f32::consts::PI / subdivision as f32;

        // calculate subdivision color
        let subdivision_color = [(color[0] - color[0]) / subdivision as f32,
                                 (color[1] - color[1]) / subdivision as f32,
                                 (color[2] - color[2]) / subdivision as f32];

        // create vertices
        let mut current = [center[0] + radius, center[1], center[2], 0.0];
        let mut current_color = color;

        for _ in 0..subdivision {
            // calculate the next point
            let next = [current[0] * std::f32::cos(subdivision_angle) - current[1] * std::f32::sin(subdivision_angle),
                        current[0] * std::f32::sin(subdivision_angle) + current[1] * std::f32::cos(subdivision_angle),
                        current[2], 0.0];

            // calculate the next color
            let next_color = [current_color[0] + subdivision_color[0],
                              current_color[1] + subdivision_color[1],
                              current_color[2] + subdivision_color[2], 0.0];

            // calculate the next two corners
            let next_c1 = [next[0] + center[0], next[1] + center[1], next[2] + center[2], 0.0];
            let next_c2 = [current[0] + center[0], current[1] + center[1], current[2] + center[2], 0.0];

            // calculate the normals
            let normal1 = Mesh::triangle_normal(current, next_c1, center);
            let normal2 = Mesh::triangle_normal(current, next_c2, center);

            // add the vertices
            vertices.push(Vertex::new(current, current_color, normal1));
            vertices.push(Vertex::new(current, current_color,
                                        [normal1[0] * -1.0, normal1[1] * -1.0, normal1[2] * -1.0, 0.0]));

            vertices.push(Vertex::new(next, next_color, normal2));
            vertices.push(Vertex::new(next, next_color,
                                        [normal2[0] * -1.0, normal2[1] * -1.0, normal2[2] * -1.0, 0.0]));

            // update current
            current = next;
            current_color = next_color;

            // add the indices
            indices.push(vertices.len() as u32 - 4);
            indices.push(vertices.len() as u32 - 3);
            indices.push(vertices.len() as u32 - 2);
            indices.push(vertices.len() as u32 - 3);
            indices.push(vertices.len() as u32 - 1);
            indices.push(vertices.len() as u32 - 2);
            indices.push(vertices.len() as u32 - 4);
            indices.push(vertices.len() as u32 - 2);
            indices.push(vertices.len() as u32 - 3);
            indices.push(vertices.len() as u32 - 3);
            indices.push(vertices.len() as u32 - 2);
            indices.push(vertices.len() as u32 - 1);
        }

        // create mesh
        Self{vertices, indices}
    }
}