use crate::graphics::color::Color;
use crate::graphics::position::Position;
use crate::graphics::normal::Normal;


#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: Position,
    pub color: Color,
    pub normal: Normal,
}

impl Vertex {
    pub fn new(position: Position, color: Color, normal: Normal) -> Self {
        Self { position, color, normal }
    }

    pub const fn new_const(position: Position, color: Color, normal: Normal) -> Self {
        Self { position, color, normal }
    }

    pub fn length(self, target: Vertex) -> f32 {
        let d = self - target;

        (d.position.x * d.position.x + d.position.y * d.position.y + d.position.z * d.position.z).sqrt()
    }

    pub fn cross(self, target: Vertex) -> Vertex {
        Vertex::new(
            self.position.cross(target.position),
            self.color * target.color / 2.0,
            self.normal.cross(target.normal),
        )
    }

    // calculate normal based on corners
    pub fn normal(self, a: Vertex, b: Vertex) -> Normal {
        (a.position - self.position).cross(b.position - self.position).into()
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

// imple subtracting two vertices
impl std::ops::Sub for Vertex {
    type Output = Vertex;

    fn sub(self, other: Vertex) -> Vertex {
        Vertex {
            position: [
                self.position[0] - other.position[0],
                self.position[1] - other.position[1],
                self.position[2] - other.position[2],
                self.position[3],
            ].into(),
            color: self.color - other.color,
            normal: self.normal - other.normal,
        }
    }
}

// implement subtracting vertex from [f32; 4]
impl std::ops::Sub<[f32; 4]> for Vertex {
    type Output = Vertex;

    fn sub(self, other: [f32; 4]) -> Vertex {
        Vertex {
            position: [
                self.position[0] - other[0],
                self.position[1] - other[1],
                self.position[2] - other[2],
                self.position[3],
            ].into(),
            color: self.color,
            normal: self.normal,
        }
    }
}

// implement division for vertex from scalar
impl std::ops::Div<f32> for Vertex {
    type Output = Vertex;

    fn div(self, other: f32) -> Vertex {
        Vertex {
            position: [
                self.position[0] / other,
                self.position[1] / other,
                self.position[2] / other,
                self.position[3],
            ].into(),
            color: self.color,
            normal: self.normal,
        }
    }
}



// implement to vertex from (x, y, z) coordinates
impl From<(f32, f32, f32)> for Vertex {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self {
            position: [x, y, z, 1.0].into(),
            color: Color::white(),
            normal: [0.0, 1.0, 0.0].into(),
        }
    }
}


// implement format
impl std::fmt::Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Vertex:
                position: {}, 
                color: {}, 
                normal: {}", 
            self.position, self.color, self.normal)
    }
}