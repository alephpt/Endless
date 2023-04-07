
use crate::graphics::normal::Normal;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    // create constant positions
    pub const fn new_const(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    // get the distance between two positions
    pub fn distance(self, target: Position) -> f32 {
        let d = self - target;

        (d.x * d.x + d.y * d.y + d.z * d.z).sqrt()
    }
    
    // get the direction to a target position
    pub fn direction(self, target: Position) -> Position {
        (self - target) / self.distance(target)
    }

    // cross product
    pub fn cross(self, target: Position) -> Position {
        Position::new(
            self.y * target.z - self.z * target.y,
            self.z * target.x - self.x * target.z,
            self.x * target.y - self.y * target.x,
            self.w * target.w - self.w * target.w,
        )
    }

    // find the next position in a given direction
    pub fn find_next(self, direction: Position, distance: f32) -> Position {
        self + direction * distance
    }

    // rotate a position around the origin across an axis
    pub fn rotate(self, angle: f32, origin: Position, axis: Position) -> Position {
        let mut position = self - origin;

        // convert angle to radians
        let (s, c) = angle.sin_cos();

        let x = position.x;
        let y = position.y;
        let z = position.z;

        let u = axis.x;
        let v = axis.y;
        let w = axis.z;

        // define rotated position around axis
        position.x = u * (u * x + v * y + w * z) * (1.0 - c) + x * c + (-w * y + v * z) * s;
        position.y = v * (u * x + v * y + w * z) * (1.0 - c) + y * c + (w * x - u * z) * s;
        position.z = w * (u * x + v * y + w * z) * (1.0 - c) + z * c + (-v * x + u * y) * s;

        position + origin
    }
}

// implement the ability to index into the position struct
impl std::ops::Index<usize> for Position {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds"),
        }
    }
}

// to position from array
impl From<[f32; 4]> for Position {
    fn from(array: [f32; 4]) -> Self {
        Self::new(array[0], array[1], array[2], array[3])
    }
}

impl From<[f32; 3]> for Position {
    fn from(array: [f32; 3]) -> Self {
        Self::new(array[0], array[1], array[2], 1.0)
    }
}

// arithmetic operators
impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

impl std::ops::Div<f32> for Position {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl std::ops::Mul<f32> for Position {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

// implement adding position with array
impl std::ops::Add<[f32; 4]> for Position {
    type Output = Self;

    fn add(self, rhs: [f32; 4]) -> Self::Output {
        Self::new(self.x + rhs[0], self.y + rhs[1], self.z + rhs[2], self.w + rhs[3])
    }
}

impl std::ops::Sub<[f32; 4]> for Position {
    type Output = Self;

    fn sub(self, rhs: [f32; 4]) -> Self::Output {
        Self::new(self.x - rhs[0], self.y - rhs[1], self.z - rhs[2], self.w - rhs[3])
    }
}

impl std::ops::Div<[f32; 4]> for Position {
    type Output = Self;

    fn div(self, rhs: [f32; 4]) -> Self::Output {
        Self::new(self.x / rhs[0], self.y / rhs[1], self.z / rhs[2], self.w / rhs[3])
    }
}

impl std::ops::Mul<[f32; 4]> for Position {
    type Output = Self;

    fn mul(self, rhs: [f32; 4]) -> Self::Output {
        Self::new(self.x * rhs[0], self.y * rhs[1], self.z * rhs[2], self.w * rhs[3])
    }
}

// implementing arithmetic of normals with positions
impl std::ops::Add<Normal> for Position {
    type Output = Self;

    fn add(self, rhs: Normal) -> Self::Output {
        Self::new(self.x + rhs[0], self.y + rhs[1], self.z + rhs[2], self.w)
    }
}

impl std::ops::Sub<Normal> for Position {
    type Output = Self;

    fn sub(self, rhs: Normal) -> Self::Output {
        Self::new(self.x - rhs[0], self.y - rhs[1], self.z - rhs[2], self.w)
    }
}

impl std::ops::Div<Normal> for Position {
    type Output = Self;

    fn div(self, rhs: Normal) -> Self::Output {
        Self::new(self.x / rhs[0], self.y / rhs[1], self.z / rhs[2], self.w)
    }
}

impl std::ops::Mul<Normal> for Position {
    type Output = Self;

    fn mul(self, rhs: Normal) -> Self::Output {
        Self::new(self.x * rhs[0], self.y * rhs[1], self.z * rhs[2], self.w)
    }
}


// implement test for rotation around an origin
#[cfg(test)]
mod tests {
    use super::*;

    // test rotating 90 degrees around the z axis
    #[test]
    fn test_rotate() {
        let position = Position::new(1.0, 0.0, 0.0, 1.0);
        let origin = Position::new(0.0, 0.0, 0.0, 1.0);
        let axis = Position::new(0.0, 0.0, 1.0, 1.0);

        let rotated = position.rotate(std::f32::consts::PI / 2.0, origin, axis);

        assert_eq!(rotated.x, 0.0);
        assert_eq!(rotated.y, 1.0);
        assert_eq!(rotated.z, 0.0);
        assert_eq!(rotated.w, 1.0);
    }

    // test rotation 45 degrees around the x and y axis
    #[test]
    fn test_rotate_2() {
        let position = Position::new(1.0, 0.0, 0.0, 1.0);
        let origin = Position::new(0.0, 0.0, 0.0, 1.0);
        let axis = Position::new(1.0, 1.0, 0.0, 1.0);

        let rotated = position.rotate(std::f32::consts::PI / 4.0, origin, axis);

        assert_eq!(rotated.x, 0.0);
        assert_eq!(rotated.y, 0.0);
        assert_eq!(rotated.z, 1.0);
        assert_eq!(rotated.w, 1.0);
    }

    // test rotation 180 degreez around x, y, z
    #[test]
    fn test_rotate_3() {
        let position = Position::new(1.0, 1.0, 1.0, 1.0);
        let origin = Position::new(0.0, 0.0, 0.0, 1.0);
        let axis = Position::new(1.0, 1.0, 1.0, 1.0);

        let rotated = position.rotate(std::f32::consts::PI, origin, axis);

        assert_eq!(rotated.x, -1.0);
        assert_eq!(rotated.y, -1.0);
        assert_eq!(rotated.z, -1.0);
        assert_eq!(rotated.w, 1.0);
    }
}