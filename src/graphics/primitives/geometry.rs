use crate::graphics::Cube;
use crate::graphics::Triangle;
use crate::graphics::Square;
use crate::graphics::Vertex;
use crate::graphics::Position;
use crate::graphics::Mesh;

pub enum Shape {
    Triangle,
    Cube,
    Square,
}

#[derive(Debug, Clone)]
pub enum Geometry {
    Triangle(Triangle),
    Cube(Cube),
    Square(Square),
}

impl Geometry {
    pub fn new(origin: Position, size: f32, shape: Shape) -> Self {
        match shape {
            Shape::Triangle => Self::Triangle(Triangle::new(origin, size)),
            Shape::Cube => Self::Cube(Cube::new(origin, size)),
            Shape::Square => Self::Square(Square::new(origin, size)),
        }
    }

    pub fn mesh(&self) -> &Mesh {
        match self {
            Self::Triangle(triangle) => triangle.mesh(),
            Self::Cube(cube) => cube.mesh(),
            Self::Square(square) => square.mesh(),
        }
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        match self {
            Self::Triangle(triangle) => triangle.vertices(),
            Self::Cube(cube) => cube.vertices(),
            Self::Square(square) => square.vertices(),
        }
    }

    pub fn indices(&self) -> &Vec<u16> {
        match self {
            Self::Triangle(triangle) => triangle.indices(),
            Self::Cube(cube) => cube.indices(),
            Self::Square(square) => square.indices(),
        }
    }

    pub fn vertex_len(&self) -> usize {
        match self {
            Self::Triangle(triangle) => triangle.vertex_len(),
            Self::Cube(cube) => cube.vertex_len(),
            Self::Square(square) => square.vertex_len(),
        }
    }

    pub fn index_len(&self) -> usize {
        match self {
            Self::Triangle(triangle) => triangle.index_len(),
            Self::Cube(cube) => cube.index_len(),
            Self::Square(square) => square.index_len(),
        }
    }

    pub fn rotate(&mut self, angle: f32, axis: Position) {
        match self {
            Self::Triangle(triangle) => triangle.rotate(angle, axis),
            Self::Cube(cube) => cube.rotate(angle, axis),
            Self::Square(square) => square.rotate(angle, axis),
        }
    }

    pub fn subdivide(&mut self, level: u32) {
        match self {
            Self::Triangle(triangle) => triangle.subdivide(level),
            Self::Cube(cube) => cube.subdivide(level),
            Self::Square(square) => square.subdivide(level),
        }
    }

    pub fn dedup(&mut self) {
        match self {
            Self::Triangle(triangle) => triangle.dedup(),
            Self::Cube(cube) => cube.dedup(),
            Self::Square(square) => square.dedup(),
        }
    }
}

// implement generic geometry trait
pub trait Geometric {
    // new geometry
    fn new(origin: Position, size: f32) -> Self;

    // get the mesh of an object
    fn mesh(&self) -> &Mesh;

    // get the vertices of the geometry
    fn vertices(&self) -> &Vec<Vertex>;

    // get the indices of the geometry
    fn indices(&self) -> &Vec<u16>;
    
    // get the number of vertices in the geometry
    fn vertex_len(&self) -> usize;

    // get the number of indices in the geometry
    fn index_len(&self) -> usize;

    // rotate the geometry around an axis
    fn rotate(&mut self, angle: f32, axis: Position);
}

impl Geometric for Triangle {
    fn new(origin: Position, size: f32) -> Self {
        Self::new(origin, size)
    }

    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn vertices(&self) -> &Vec<Vertex> {
        &self.mesh.vertices
    }
    fn indices(&self) -> &Vec<u16> {
        &self.mesh.indices
    }
    fn vertex_len(&self) -> usize {
        self.mesh.vertices.len()
    }
    fn index_len(&self) -> usize {
        self.mesh.indices.len()
    }
    fn rotate(&mut self, angle: f32, axis: Position) {
        self.rotate(angle, axis);
    }
}

impl Geometric for Cube {
    fn new(origin: Position, size: f32) -> Self {
        Self::new(origin, size)
    }

    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn vertices(&self) -> &Vec<Vertex> {
        &self.mesh.vertices
    }
    fn indices(&self) -> &Vec<u16> {
        &self.mesh.indices
    }
    fn vertex_len(&self) -> usize {
        self.mesh.vertices.len()
    }
    fn index_len(&self) -> usize {
        self.mesh.indices.len()
    }
    fn rotate(&mut self, angle: f32, axis: Position) {
        self.rotate(angle, axis);
    }
}

impl Geometric for Square {
    fn new(origin: Position, size: f32) -> Self {
        Self::new(origin, size)
    }

    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn vertices(&self) -> &Vec<Vertex> {
        &self.mesh.vertices
    }
    fn indices(&self) -> &Vec<u16> {
        &self.mesh.indices
    }
    fn vertex_len(&self) -> usize {
        self.mesh.vertices.len()
    }
    fn index_len(&self) -> usize {
        self.mesh.indices.len()
    }
    fn rotate(&mut self, angle: f32, axis: Position) {
        self.rotate(angle, axis);
    }
}