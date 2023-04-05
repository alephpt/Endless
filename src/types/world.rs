
struct Location {
    x: f32,
    y: f32,
    z: f32,
}

struct Bounds {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    min_z: f32,
    max_z: f32,
}

impl Bounds {
    fn new(min_x: f32, max_x: f32, min_y: f32, max_y: f32, min_z: f32, max_z: f32) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        }
    }

    fn contains(&self, location: &Location) -> bool {
        location.x >= self.min_x
            && location.x <= self.max_x
            && location.y >= self.min_y
            && location.y <= self.max_y
            && location.z >= self.min_z
            && location.z <= self.max_z
    }

    fn wrap_location(&self, location: &Location) -> Location {
        let mut x = location.x;
        let mut y = location.y;
        let mut z = location.z;

        if x < self.min_x {
            x = self.max_x;
        } else if x > self.max_x {
            x = self.min_x;
        }
        
        if y < self.min_y {
            y = self.max_y;
        } else if y > self.max_y {
            y = self.min_y;
        }

        if z < self.min_z {
            z = self.max_z;
        } else if z > self.max_z {
            z = self.min_z;
        }

        Location { x, y, z }
    }
}

impl Location {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn distance(&self, other: &Self) -> f32 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        (x * x + y * y + z * z).sqrt()
    }

    fn direction(&self, other: &Self) -> Self {
        let distance = self.distance(other);
        Self {
            x: (other.x - self.x) / distance,
            y: (other.y - self.y) / distance,
            z: (other.z - self.z) / distance,
        }
    }

    fn update_location(&self, bounds: &Bounds){
        if self.x < bounds.min_x {
            self.x = bounds.max_x;
        } else if self.x > bounds.max_x {
            self.x = bounds.min_x;
        }
        
        if self.y < bounds.min_y {
            self.y = bounds.max_y;
        } else if self.y > bounds.max_y {
            self.y = bounds.min_y;
        }

        if self.z < bounds.min_z {
            self.z = bounds.max_z;
        } else if self.z > bounds.max_z {
            self.z = bounds.min_z;
        }
    }
}
