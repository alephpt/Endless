use crate::graphics::position::Position;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Normal(pub [f32; 3]);

impl Normal {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }

    pub const fn new_const(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }

    pub fn to_vec4(self) -> Position {
        Position::new(self.0[0], self.0[1], self.0[2], 0.0)
    }

    // find the cross product of two normals (directions)
    pub fn cross(self, target: Self) -> Self {
        Self([
            self.0[1] * target.0[2] - self.0[2] * target.0[1],
            self.0[2] * target.0[0] - self.0[0] * target.0[2],
            self.0[0] * target.0[1] - self.0[1] * target.0[0],
        ])
    }
}

// implement indexing into normal
impl std::ops::Index<usize> for Normal {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0[0],
            1 => &self.0[1],
            2 => &self.0[2],
            _ => panic!("Index out of bounds"),
        }
    }
}

// implement normal from array
impl From<[f32; 3]> for Normal {
    fn from(array: [f32; 3]) -> Self {
        Self(array)
    }
}

// implement normal from (x, y, z) coordinates
impl From<(f32, f32, f32)> for Normal {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self([x, y, z])
    }
}

// implement normal from position
impl From<Position> for Normal {
    fn from(position: Position) -> Self {
        Self([position.x, position.y, position.z])
    }
}

// implement array from normal
impl From<Normal> for [f32; 3] {
    fn from(normal: Normal) -> Self {
        normal.0
    }
}

// implement (x, y, z) from normal
impl From<Normal> for (f32, f32, f32) {
    fn from(normal: Normal) -> Self {
        (normal.0[0], normal.0[1], normal.0[2])
    }
}

// implement Position from normal
impl From<Normal> for Position {
    fn from(normal: Normal) -> Self {
        Position::new(normal.0[0], normal.0[1], normal.0[2], 1.0)
    }
}

// check if two normals are equal
impl PartialEq for Normal {
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0] && self.0[1] == other.0[1] && self.0[2] == other.0[2]
    }
}

// arithmetic operators
impl std::ops::Add for Normal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1], self.0[2] + rhs.0[2]])
    }
}

impl std::ops::Sub for Normal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self([self.0[0] - rhs.0[0], self.0[1] - rhs.0[1], self.0[2] - rhs.0[2]])
    }
}

impl std::ops::Div<f32> for Normal {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self([self.0[0] / rhs, self.0[1] / rhs, self.0[2] / rhs])
    }
}

impl std::ops::Mul<f32> for Normal {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}

// implement adding normal with array
impl std::ops::Add<[f32; 3]> for Normal {
    type Output = Self;

    fn add(self, rhs: [f32; 3]) -> Self::Output {
        Self([self.0[0] + rhs[0], self.0[1] + rhs[1], self.0[2] + rhs[2]])
    }
}

impl std::ops::Sub<[f32; 3]> for Normal {
    type Output = Self;

    fn sub(self, rhs: [f32; 3]) -> Self::Output {
        Self([self.0[0] - rhs[0], self.0[1] - rhs[1], self.0[2] - rhs[2]])
    }
}

// implement adding normal with (x, y, z) coordinates
impl std::ops::Add<(f32, f32, f32)> for Normal {
    type Output = Self;

    fn add(self, rhs: (f32, f32, f32)) -> Self::Output {
        Self([self.0[0] + rhs.0, self.0[1] + rhs.1, self.0[2] + rhs.2])
    }
}

impl std::ops::Sub<(f32, f32, f32)> for Normal {
    type Output = Self;

    fn sub(self, rhs: (f32, f32, f32)) -> Self::Output {
        Self([self.0[0] - rhs.0, self.0[1] - rhs.1, self.0[2] - rhs.2])
    }
}

// implement adding normal with position
impl std::ops::Add<Position> for Normal {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output {
        Self([self.0[0] + rhs.x, self.0[1] + rhs.y, self.0[2] + rhs.z])
    }
}

impl std::ops::Sub<Position> for Normal {
    type Output = Self;

    fn sub(self, rhs: Position) -> Self::Output {
        Self([self.0[0] - rhs.x, self.0[1] - rhs.y, self.0[2] - rhs.z])
    }
}

// implement format
impl std::fmt::Display for Normal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0[0], self.0[1], self.0[2])
    }
}