
use rand::prelude::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color(pub [f32; 4]);

// define a series of primary color 
// Color::black(), Color::white(), Color::red(), Color::green(), Color::blue(), Color::yellow()
impl Color {
    pub fn black() -> Self { Self([0.0, 0.0, 0.0, 1.0]) }
    pub fn white() -> Self { Self([1.0, 1.0, 1.0, 1.0]) }
    pub fn red() -> Self { Self([1.0, 0.0, 0.0, 1.0]) }
    pub fn green() -> Self { Self([0.0, 1.0, 0.0, 1.0]) }
    pub fn blue() -> Self { Self([0.0, 0.0, 1.0, 1.0]) }
    pub fn yellow() -> Self { Self([1.0, 1.0, 0.0, 1.0]) }
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self([rng.gen(), rng.gen(), rng.gen(), 1.0])
    }
}

// impl add for Color
impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
            self.0[3] + other.0[3],
        ])
    }
}

// impl sub for Color
impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self([
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
            self.0[3] - other.0[3],
        ])
    }
}

// impl mul for Color
impl std::ops::Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self([
            self.0[0] * other.0[0],
            self.0[1] * other.0[1],
            self.0[2] * other.0[2],
            self.0[3] * other.0[3],
        ])
    }
}

// impl div for Color
impl std::ops::Div for Color {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self([
            self.0[0] / other.0[0],
            self.0[1] / other.0[1],
            self.0[2] / other.0[2],
            self.0[3] / other.0[3],
        ])
    }
}

// impl add for Color and [f32; 4]
impl std::ops::Add<[f32; 4]> for Color {
    type Output = Self;

    fn add(self, other: [f32; 4]) -> Self {
        Self([
            self.0[0] + other[0],
            self.0[1] + other[1],
            self.0[2] + other[2],
            self.0[3] + other[3],
        ])
    }
}

// impl sub for Color and [f32; 4]
impl std::ops::Sub<[f32; 4]> for Color {
    type Output = Self;

    fn sub(self, other: [f32; 4]) -> Self {
        Self([
            self.0[0] - other[0],
            self.0[1] - other[1],
            self.0[2] - other[2],
            self.0[3] - other[3],
        ])
    }
}

// impl mul for Color and [f32; 4]
impl std::ops::Mul<[f32; 4]> for Color {
    type Output = Self;

    fn mul(self, other: [f32; 4]) -> Self {
        Self([
            self.0[0] * other[0],
            self.0[1] * other[1],
            self.0[2] * other[2],
            self.0[3] * other[3],
        ])
    }
}

// impl div for Color and [f32; 4]
impl std::ops::Div<[f32; 4]> for Color {
    type Output = Self;

    fn div(self, other: [f32; 4]) -> Self {
        Self([
            self.0[0] / other[0],
            self.0[1] / other[1],
            self.0[2] / other[2],
            self.0[3] / other[3],
        ])
    }
}

// impl from [{float}; 4] for Color
impl From<[f32; 4]> for Color {
    fn from([r, g, b, a]: [f32; 4]) -> Self {
        Self([r, g, b, a])
    }
}

// impl into [{float}; 4] for Color
impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        self.0
    }
}