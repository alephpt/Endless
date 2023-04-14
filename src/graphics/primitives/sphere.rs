
use crate::graphics::{Geometry, Mesh, Position};

// TODO: Extrapolate this out into a generic / trait
#[derive(Debug, Clone)]
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

pub struct Spherical {
    pub sphere_type: Sphere,
    pub radius: f32,
    pub origin: Position,
    pub mesh: Mesh,
}

impl Spherical {
    pub fn sphere(radius: f32, origin: Position, sphere_type: SphereType) -> Spherical {
        Spherical {
            sphere_type,
            radius,
            origin,
            mesh: Sphere::mesh(radius, origin, sphere_type),
        }
    }

    fn mesh(self, radius: f32, origin: Position, sphere_type: SphereType) -> Mesh {
        match sphere_type {
            SphereType::UVSphere => Self::uv_sphere(radius, origin),
            SphereType::Icosahedron => Self::icosahedron(radius, origin),
            SphereType::SpherifiedCube => Self::spherified_cube(radius, origin),
        }
    }

    fn uv_sphere(radius: f32, origin: Position) -> Mesh {
        // generate vertices
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let mut vertex_index = 0;
        let mut index_index = 0;

        let mut phi = 0.0;
        let mut theta = 0.0;

    }

    fn icosahedron(radius: f32, origin: Position) -> Mesh {

    }

    fn spherified_cube(radius: f32, origin: Position) -> Mesh {

    }
}