

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 1200;

// store 
pub struct pov {
    pub center: (f32, f32, f32),
    pub bottom_left: (f32, f32, f32),
    pub bottom_right: (f32, f32, f32),
    pub top_center: (f32, f32, f32),
}

impl pov {
    pub fn new() -> pov {
        pov {
            center: (SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, 0.0),
            bottom_left: (0.0, SCREEN_HEIGHT, 0.0),
            bottom_right: (SCREEN_WIDTH, SCREEN_HEIGHT, 0.0),
            top_center: (SCREEN_WIDTH / 2, 0.0, 0.0),
        }
    }

    // TODO: draws lines from the bottom_left, bottom_right, and top_center to the center
}

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}