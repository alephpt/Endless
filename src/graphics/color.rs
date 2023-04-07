
use rand::prelude::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color{
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    // define a series of primary colors
    pub const fn black() -> Self { Self{ r: 0.0, g: 0.0, b: 0.0, a: 1.0 } }
    pub const fn white() -> Self { Self{ r: 1.0, g: 1.0, b: 1.0, a: 1.0 } }
    pub const fn red() -> Self { Self{ r: 1.0, g: 0.0, b: 0.0, a: 1.0 } }
    pub const fn green() -> Self { Self{ r: 0.0, g: 1.0, b: 0.0, a: 1.0 } }
    pub const fn blue() -> Self { Self{ r: 0.0, g: 0.0, b: 1.0, a: 1.0 } }
    
    // define a series of secondary colors
    pub const fn yellow() -> Self { Self{ r: 1.0, g: 1.0, b: 0.0, a: 1.0 } }
    pub const fn cyan() -> Self { Self{ r: 0.0, g: 1.0, b: 1.0, a: 1.0 } }
    pub const fn magenta() -> Self { Self{ r: 1.0, g: 0.0, b: 1.0, a: 1.0 } }
    pub const fn orange() -> Self { Self{ r: 1.0, g: 0.5, b: 0.0, a: 1.0 } }
    pub const fn purple() -> Self { Self{ r: 0.5, g: 0.0, b: 0.5, a: 1.0 } }
    pub const fn pink() -> Self { Self{ r: 1.0, g: 0.0, b: 0.5, a: 1.0 } }
    pub const fn brown() -> Self { Self{ r: 0.5, g: 0.0, b: 0.0, a: 1.0 } }
    pub const fn turquoise() -> Self { Self{ r: 0.0, g: 0.5, b: 0.5, a: 1.0 } }
    pub const fn lime() -> Self { Self{ r: 0.0, g: 1.0, b: 0.0, a: 1.0 } }
    pub const fn maroon() -> Self { Self{ r: 0.5, g: 0.0, b: 0.0, a: 1.0 } }
    pub const fn olive() -> Self { Self{ r: 0.5, g: 0.5, b: 0.0, a: 1.0 } }
    pub const fn teal() -> Self { Self{ r: 0.0, g: 0.5, b: 0.5, a: 1.0 } }
    pub const fn navy() -> Self { Self{ r: 0.0, g: 0.0, b: 0.5, a: 1.0 } }
    pub const fn violet() -> Self { Self{ r: 0.5, g: 0.0, b: 0.5, a: 1.0 } }
    
    // define a series of tertiary colors
    pub const fn gray() -> Self { Self{ r: 0.5, g: 0.5, b: 0.5, a: 1.0 } }
    pub const fn silver() -> Self { Self{ r: 0.75, g: 0.75, b: 0.75, a: 1.0 } }
    pub const fn gold() -> Self { Self{ r: 1.0, g: 0.84, b: 0.0, a: 1.0 } }
    
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            r: rng.gen_range(0.0..1.0),
            g: rng.gen_range(0.0..1.0),
            b: rng.gen_range(0.0..1.0),
            a: 1.0,
        }
    }

    // define a series of transparency functions
    pub fn transparent(&mut self) -> Self { 
        self.a = 0.0;
        *self
    }

    pub fn semi_transparent(&mut self) -> Self { 
        self.a = 0.25;
        *self
    }

    pub fn translucent(&mut self) -> Self { 
        self.a = 0.5;
        *self
    }

    pub fn semi_opaque(&mut self) -> Self { 
        self.a = 0.75;
        *self
    }

    pub fn opaque(&mut self) -> Self { 
        self.a = 1.0;
        *self
    }
}

// impl addassign for Color
impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
        self.a += other.a;
    }
}

// impl add for Color
impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let r = self.r + other.r;
        let g = self.g + other.g;
        let b = self.b + other.b;
        let a = self.a + other.a;

        Self { r, g, b, a }
    }
}

// impl sub for Color
impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let r = self.r - other.r;
        let g = self.g - other.g;
        let b = self.b - other.b;
        let a = self.a - other.a;

        Self { r, g, b, a }
    }
}

// impl mul for Color
impl std::ops::Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let r = self.r * other.r;
        let g = self.g * other.g;
        let b = self.b * other.b;
        let a = self.a * other.a;

        Self { r, g, b, a }
    }
}

// impl mul for Color and f32
impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let r = self.r * other;
        let g = self.g * other;
        let b = self.b * other;
        let a = self.a * other;

        Self { r, g, b, a }
    }
}

// impl div for Color
impl std::ops::Div for Color {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let r = self.r / other.r;
        let g = self.g / other.g;
        let b = self.b / other.b;
        let a = self.a / other.a;

        Self { r, g, b, a }
    }
}

// impl add for Color and [f32; 4]
impl std::ops::Add<[f32; 4]> for Color {
    type Output = Self;

    fn add(self, other: [f32; 4]) -> Self {
        let r = self.r + other[0];
        let g = self.g + other[1];
        let b = self.b + other[2];
        let a = self.a + other[3];

        Self { r, g, b, a }
    }
}

// impl sub for Color and [f32; 4]
impl std::ops::Sub<[f32; 4]> for Color {
    type Output = Self;

    fn sub(self, other: [f32; 4]) -> Self {
        let r = self.r - other[0];
        let g = self.g - other[1];
        let b = self.b - other[2];
        let a = self.a - other[3];

        Self { r, g, b, a }
    }
}

// impl mul for Color and [f32; 4]
impl std::ops::Mul<[f32; 4]> for Color {
    type Output = Self;

    fn mul(self, other: [f32; 4]) -> Self {
        let r = self.r * other[0];
        let g = self.g * other[1];
        let b = self.b * other[2];
        let a = self.a * other[3];

        Self { r, g, b, a }
    }
}

// impl div for Color and [f32; 4]
impl std::ops::Div<[f32; 4]> for Color {
    type Output = Self;

    fn div(self, other: [f32; 4]) -> Self {
        let r = self.r / other[0];
        let g = self.g / other[1];
        let b = self.b / other[2];
        let a = self.a / other[3];

        Self { r, g, b, a }
    }
}

// div color by f32
impl std::ops::Div<f32> for Color {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        let r = self.r / other;
        let g = self.g / other;
        let b = self.b / other;
        let a = self.a / other;

        Self { r, g, b, a }
    }
}

// impl into [{float}; 4] for Color
impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

// impl into Color for [{float}; 4]
impl Into<Color> for [f32; 4] {
    fn into(self) -> Color {
        Color { r: self[0], g: self[1], b: self[2], a: self[3] }
    }
}

// impl the ability to index into Color
impl std::ops::Index<usize> for Color {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            3 => &self.a,
            _ => panic!("Index out of bounds"),
        }
    }
}

// implement format
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Color {{ r: {}, g: {}, b: {}, a: {} }}", self.r, self.g, self.b, self.a)
    }
}