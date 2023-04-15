
use crate::graphics::{Geometry, Mesh, Position, Vertex, Color};

// TODO: Extrapolate this out into a generic / trait
#[derive(Debug, Copy, Clone)]
pub enum Sphere {
    UVSphere,
    Icosahedron,
    SpherifiedCube,
}

impl Sphere {
    pub fn new(self, radius: f32, origin: Position) -> Geometry {
        match self {
            Sphere::UVSphere => Geometry::Sphere(Spherical::sphere(radius, origin, self)),
            Sphere::Icosahedron => Geometry::Sphere(Spherical::sphere(radius, origin, self)),
            Sphere::SpherifiedCube => Geometry::Sphere(Spherical::sphere(radius, origin, self)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Spherical {
    pub sphere_type: Sphere,
    pub radius: f32,
    pub origin: Position,
    pub mesh: Mesh,
}

impl Spherical {
    pub fn sphere(radius: f32, origin: Position, sphere_type: Sphere) -> Spherical {
        Spherical {
            sphere_type,
            radius,
            origin,
            mesh: Spherical::mesh(radius, origin, sphere_type),
        }
    }

    fn mesh(radius: f32, origin: Position, sphere_type: Sphere) -> Mesh {
        match sphere_type {
            Sphere::UVSphere => Self::uv_sphere(radius, origin),
            Sphere::Icosahedron => Self::icosahedron(radius, origin),
            Sphere::SpherifiedCube => Self::spherified_cube(radius, origin),
        }
    }

    fn calculate_spiral_color(i: u16, j: u16, stacks: u16, sectors: u16) -> Color{
        let latitude = (std::f32::consts::PI * i as f32) / stacks as f32;
        let longitude = (2.0 * std::f32::consts::PI * j as f32) / sectors as f32;
    
        let angle = (longitude - latitude).abs() * 2.0;
        let frequency = 6.0; // You can adjust the frequency to control the number of spirals
    
        let t = (i as f32 / stacks as f32 + angle * frequency) % (2.0 * std::f32::consts::PI);
    
        let r = t.sin() * 0.5 + 0.5 - (i * j) as f32;
        let g = ((t + std::f32::consts::PI / 2.0) % (2.0 * std::f32::consts::PI)).sin() * 0.5 + 0.5;
        let b = t.cos() * 0.5 + 0.5 + (i + 1 * j + 1) as f32 / (stacks * sectors / 2) as f32;
    
        Color::new(r, g, b, 1.0)
    }

    fn uv_sphere(radius: f32, origin: Position) -> Mesh {
        // generate vertices
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        let sectors = 32;
        let stacks = 32;

        let sector_step = 2.0 * std::f32::consts::PI / sectors as f32;
        let stack_step = std::f32::consts::PI / stacks as f32;

        // generate vertices
        for i in 0..=stacks {
            let stack_angle = std::f32::consts::PI / 2.0 - i as f32 * stack_step;
            let xy = radius * stack_angle.cos();
            let z = radius * stack_angle.sin();

            for j in 0..=sectors {
                let sector_angle = j as f32 * sector_step;

                let x = xy * sector_angle.cos();
                let y = xy * sector_angle.sin();

                vertices.push(Vertex {
                    position: Position::new(origin.x + x, origin.y + y, origin.z + z, 1.0),
                    color: Self::calculate_spiral_color(i, j, stacks, sectors),
                    normal: (0.0, 0.0, 1.0).into(),
                });
            }
        }

        // generate indices
        for i in 0..stacks {
            let k1 = i * (sectors + 1);
            let k2 = k1 + sectors + 1;

            for j in 0..sectors {
                indices.push(k1 + j);
                indices.push(k2 + j);
                indices.push(k2 + j + 1);

                indices.push(k1 + j);
                indices.push(k2 + j + 1);
                indices.push(k1 + j + 1);
            }
        }

        Mesh::new(vertices, indices)
    }

    fn icosahedron(radius: f32, origin: Position) -> Mesh {
        let t = (1.0 + 5.0_f32.sqrt()) / 2.0;

        let verts = vec![
            -1.0,  t,  0.0,
             1.0,  t,  0.0,
            -1.0, -t,  0.0,
            1.0,  -t,  0.0,
            0.0,  -1.0,  t,
            0.0,   1.0,  t,
            0.0,  -1.0, -t,
            0.0,   1.0, -t,
            t,   0.0, -1.0,
            t,   0.0,  1.0,
            -t,  0.0, -1.0,
            -t,  0.0,  1.0,

        ];

        let indices = vec![
            0,   11,    5,
            0,    5,    1,
            0,    1,    7,
            0,    7,   10,
            0,   10,   11,
            1,    5,    9,
            5,   11,    4,
            11,  10,    2,
            10,   7,    6,
            7,    1,    8,
            3,    9,    4,
            3,    4,    2,
            3,    2,    6,
            3,    6,    8,
            3,    8,    9,
            4,    9,    5,
            2,    4,   11,
            6,    2,   10,
            8,    6,    7,
            9,    8,    1,

        ];

        let mut vertices: Vec<Vertex> = Vec::new();

        for i in 0..verts.len() / 3 {
            vertices.push(Vertex {
                position: Position {
                    x: verts[i * 3] * radius + origin.x,
                    y: verts[i * 3 + 1] * radius + origin.y,
                    z: verts[i * 3 + 2] * radius + origin.z,
                    w: 1.0,
                },
                color: Color::blue(),
                normal: (0.0, 0.0, 1.0).into(),
            });
        }

        Mesh::new(vertices, indices)
    }

    fn spherified_cube(radius: f32, origin: Position) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        fn spherify(pos: Position, radius: f32) -> Position {
            let x2 = pos.x * pos.x;
            let y2 = pos.y * pos.y;
            let z2 = pos.z * pos.z;
        
            let spherified_pos = (
                pos.x * f32::sqrt(1.0 - y2 / 2.0 - z2 / 2.0 + y2 * z2 / 3.0),
                pos.y * f32::sqrt(1.0 - z2 / 2.0 - x2 / 2.0 + z2 * x2 / 3.0),
                pos.z * f32::sqrt(1.0 - x2 / 2.0 - y2 / 2.0 + x2 * y2 / 3.0),
            );

            Position::new(
                spherified_pos.0 * radius, 
                spherified_pos.1 * radius, 
                spherified_pos.2 * radius,
                1.0)
        }

        let subdivisions = 8;

        for s in 0..6{
            for i in 0..subdivisions{
                for j in 0..subdivisions{
                    let (sx, sy) = (i as f32 / subdivisions as f32 * 2.0 - 1.0, j as f32 / subdivisions as f32 * 2.0 - 1.0);
                    let pos = match s {
                        0 => Position::new(1.0, sx, sy, 1.0),
                        1 => Position::new(-1.0, sx, sy, 1.0),
                        2 => Position::new(sx, 1.0, sy, 1.0),
                        3 => Position::new(sx, -1.0, sy, 1.0),
                        4 => Position::new(sx, sy, 1.0, 1.0),
                        5 => Position::new(sx, sy, -1.0, 1.0),
                        _ => unreachable!(),
                    };
                    
                    vertices.push(Vertex {
                        position: spherify(pos, radius) + origin,
                        color: Color::blue(),
                        normal: (0.0, 0.0, 1.0).into(),
                    });

                    // determine index of vertex
                    if i < subdivisions - 1 && j < subdivisions - 1 {
                        let index = (s * subdivisions * subdivisions) + (i * subdivisions) + j;
                        indices.extend_from_slice(&[
                            index,
                            index + subdivisions + 1,
                            index + 1,
                            index + 1,
                            index + subdivisions + 1,
                            index + subdivisions ,
                        ])
                    }
                }
            }
        }

        Mesh::new(vertices, indices)
    }

    pub fn subdivide(&mut self, n_subdivisions: u32) {
        match self.sphere_type {
            Sphere::UVSphere => self.uv_subdivide(n_subdivisions),
            Sphere::Icosahedron => self.icosahedron_subdivide(n_subdivisions),
            Sphere::SpherifiedCube => self.spherified_cube_subdivide(n_subdivisions),
        }
    }

    fn uv_subdivide(&self, _subdivisions: u32) {
        return
    }

    fn icosahedron_subdivide(&self, _subdivisions: u32) {
        return
    }

    fn spherified_cube_subdivide(&self, _subdivisions: u32) {
        return
    }

    fn dedup(&mut self) {
        self.mesh.dedup();
    }
}