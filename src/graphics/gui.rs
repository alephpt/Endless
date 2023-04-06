

const SCREEN_WIDTH: u32 = 100;
const SCREEN_HEIGHT: u32 = 50;

#[derive(Debug)]
pub struct ShipUI {
    pub center: (f32, f32, f32),
    // pentagonal retical that comes from screens edges to the center
    pub top_center: (f32, f32, f32),
    pub mid_left: (f32, f32, f32),
    pub bottom_left: (f32, f32, f32),
    pub bottom_right: (f32, f32, f32),
    pub mid_right: (f32, f32, f32),
}

impl ShipUI {
    pub fn new() -> Self {
        const POINTS: u32 = 5;

        let center: (f32, f32, f32) = (SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0, 0.0);

        // calculate pentagonal pyramid lines for ship gui
        // with the center as the base and each point landing on the edge of the screen
        let points: Vec<(f32, f32, f32)> = (0..POINTS)
            .map(|i| {
                let angle_degrees = 52.0 as f32;
                let angle_radians = angle_degrees.to_radians();

                let screen_width = SCREEN_WIDTH as f32;
                let screen_height = SCREEN_HEIGHT as f32;
                let center_x = screen_width / 2.0;
                let center_y = screen_height / 2.0;

                let x = match i {
                    0 => screen_width / 2.0,
                    1 => 1.0,
                    2 => center_x - (center_x * angle_radians.cos()),
                    3 => center_x + (center_x * angle_radians.cos()),
                    4 => screen_width - 1.0,
                    _ => 0.0,
                };

                let y = match i {
                    0 => 1.0,
                    1 | 4 => center_y * angle_radians.sin(),
                    _ => screen_height - 1.0,
                };
                (x, y, 0.0)
            }) 
            .collect();

        
        Self {
            center: center,
            top_center: points[0],
            mid_left: points[1],
            bottom_left: points[2],
            bottom_right: points[3],
            mid_right: points[4],
        }
    }

    // draw the ship gui in ascii 
    pub fn plot(&self) {
        let mut screen: Vec<Vec<char>> = vec![vec![' '; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

        // draw the center point
        screen[self.center.1 as usize][self.center.0 as usize] = 'X';

        // draw the pentagonal pyramid
        screen[self.top_center.1 as usize][self.top_center.0 as usize] = '1';
        screen[self.mid_left.1 as usize][self.mid_left.0 as usize] = '2';
        screen[self.bottom_left.1 as usize][self.bottom_left.0 as usize] = '3';
        screen[self.bottom_right.1 as usize][self.bottom_right.0 as usize] = '4';
        screen[self.mid_right.1 as usize][self.mid_right.0 as usize] = '5';

        // print the screen
        for row in screen {
            for col in row {
                print!("{}", col);
            }
            println!();
        }
    }
}

// implement formatting for ShipGUI
impl std::fmt::Display for ShipUI {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Ship GUI:
            center: {:?}
            top_center: {:?}
            mid_left: {:?}
            bottom_left: {:?}
            bottom_right: {:?}
            mid_right: {:?}",
            self.center, self.top_center, self.mid_left, self.bottom_left, self.bottom_right, self.mid_right
        )
    }
}

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}
