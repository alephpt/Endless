
use crate::graphics::Mesh;
use crate::graphics::Vertex;
use crate::graphics::Color;
use crate::graphics::Line;
use crate::graphics::Ring;

const SCREEN_WIDTH: u32 = 100;
const SCREEN_HEIGHT: u32 = 50;
const CIRCLE_RADIUS: f32 = 7.0;

#[derive(Debug)]
pub struct ShipUIPoints {
    pub center_point: Vertex,
    // pentagonal retical that comes from screens edges to the center
    pub top_center_point: Vertex,
    pub mid_left_point: Vertex,
    pub bottom_left_point: Vertex,
    pub bottom_right_point: Vertex,
    pub mid_right_point: Vertex,
}

#[derive(Debug)]
pub struct ShipUI {
    pub center_circle: Mesh,
    pub top_center_line: Mesh,
    pub mid_left_line: Mesh,
    pub bottom_left_line: Mesh,
    pub bottom_right_line: Mesh,
    pub mid_right_line: Mesh,
}

impl ShipUIPoints {
    pub fn new() -> Self {
        const POINTS: u32 = 5;

        let center = Vertex::new(
                                [SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0, 0.0, 0.0].into(), 
                                Color::pink(), 
                                [0.0, 0.0, 1.0].into()
                            );

        // calculate pentagonal pyramid lines for ship gui
        // with the center as the base and each point landing on the edge of the screen
        let vertices: Vec<Vertex> = (0..POINTS)
            .map(|i| {
                let angle_degrees = 52.0 as f32;
                let angle_radians = angle_degrees.to_radians();

                let screen_width = SCREEN_WIDTH as f32;
                let screen_height = SCREEN_HEIGHT as f32;
                let center_x = screen_width / 2.0;
                let center_y = screen_height / 2.0;

                let x = match i {
                    0 => screen_width / 2.0,
                    1 => 5.0,
                    2 => center_x - (center_x * angle_radians.cos()) - CIRCLE_RADIUS / 1.3,
                    3 => center_x + (center_x * angle_radians.cos()) + CIRCLE_RADIUS / 1.3,
                    4 => screen_width - 5.0,
                    _ => 0.0,
                };

                let y = match i {
                    0 => 5.0,
                    1 | 4 => center_y * angle_radians.sin() - CIRCLE_RADIUS / 1.3,
                    2 | 3 => center_y * angle_radians.sin() + CIRCLE_RADIUS / 1.3,
                    _ => screen_height - 5.0,
                };
                Vertex::new([x, y, 0.0, 0.0].into(), Color::teal(), [0.0, 0.0, 1.0].into())
            }) 
            .collect();

        Self {
            center_point: center,
            top_center_point: vertices[0],
            mid_left_point: vertices[1],
            bottom_left_point: vertices[2],
            bottom_right_point: vertices[3],
            mid_right_point: vertices[4],
        }
    }

    // draw the ship gui in ascii 
    pub fn plot(&self) {
        let mut screen: Vec<Vec<char>> = vec![vec![' '; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

        // draw the center point
        screen[self.center_point.position.x as usize][self.center_point.position.y as usize] = 'X';

        // draw the pentagonal pyramid
        screen[self.top_center_point.position.x as usize][self.top_center_point.position.y as usize] = '1';
        screen[self.mid_left_point.position.x as usize][self.mid_left_point.position.y as usize] = '2';
        screen[self.bottom_left_point.position.x as usize][self.bottom_left_point.position.y as usize] = '3';
        screen[self.bottom_right_point.position.x as usize][self.bottom_right_point.position.y as usize] = '4';
        screen[self.mid_right_point.position.x as usize][self.mid_right_point.position.y as usize] = '5';

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
impl std::fmt::Display for ShipUIPoints {
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
            self.center_point, self.top_center_point, self.mid_left_point, self.bottom_left_point, self.bottom_right_point, self.mid_right_point
        )
    }
}

impl ShipUI {
    pub fn new() -> Self {
        let points = ShipUIPoints::new();

        // create the two end points of a line, minus the size of the center circle
        let top_center_line = Line::new(points.top_center_point, points.center_point, 2.0, 5).mesh;
        let mid_left_line = Line::new(points.mid_left_point, points.center_point, 2.0, 5).mesh;
        let bottom_left_line = Line::new(points.bottom_left_point, points.center_point, 2.0, 5).mesh;
        let bottom_right_line = Line::new(points.bottom_right_point, points.center_point, 2.0, 5).mesh;
        let mid_right_line = Line::new(points.mid_right_point, points.center_point, 2.0, 5).mesh;
        let center_circle = Ring::new(points.center_point.position, CIRCLE_RADIUS, 2.0, 15, Color::white()).mesh;

        Self {
            center_circle,
            top_center_line,
            mid_left_line,
            bottom_left_line,
            bottom_right_line,
            mid_right_line,
        }
    }


    // calculate lines and circle and return them as a single vector
    pub fn calculate_points(&self) -> Vec<(i32, i32)> {
        // combine all lines into one vector
        let mut all_lines: Vec<(i32, i32)> = Vec::new();

        all_lines.extend(self.top_center_line.vertices.iter().map(|i| (i.position.x as i32, i.position.y as i32)));
        all_lines.extend(self.mid_left_line.vertices.iter().map(|i| (i.position.x as i32, i.position.y as i32)));
        all_lines.extend(self.bottom_left_line.vertices.iter().map(|i| (i.position.x as i32, i.position.y as i32)));
        all_lines.extend(self.bottom_right_line.vertices.iter().map(|i| (i.position.x as i32, i.position.y as i32)));
        all_lines.extend(self.mid_right_line.vertices.iter().map(|i| (i.position.x as i32, i.position.y as i32)));
        all_lines.extend(self.center_circle.vertices.iter().map(|i| (i.position.x as i32, i.position.y as i32)));

        all_lines
    }

    // plot the ship UI to the screen
    pub fn plot(&self) {
        // create blank list 
        let mut screen: Vec<Vec<char>> = vec![vec![' '; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

        // calculate all the points
        let all_points = self.calculate_points();

        // plot the center point
        screen[SCREEN_HEIGHT as usize / 2][SCREEN_WIDTH as usize / 2] = 'X';

        // plot all the points
        for point in all_points {
            screen[point.0 as usize][point.1 as usize] = 'X';
        }

        // print the screen
        for i in screen {
            for j in i {
                print!("{}", j);
            }
            println!();
        }
    }
}

// implement formatting for ShipGUI
// printing out the start and end vectors of each line
impl std::fmt::Display for ShipUI {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Ship GUI:
            center_circle:
                start: {}, {}
                end: {}, {}
            top_center_line: 
                start: {}, {}
                end: {}, {}
            mid_left_line:
                start: {}, {}
                end: {}, {}
            bottom_left_line:
                start: {}, {}
                end: {}, {}
            bottom_right_line:
                start: {}, {}
                end: {}, {}
            mid_right_line:
                start: {}, {}
                end: {}, {}",
            self.center_circle.vertices[0].position.x, self.center_circle.vertices[0].position.y,
            self.center_circle.vertices[self.center_circle.vertices.len() / 2].position.x, self.center_circle.vertices[self.center_circle.vertices.len() / 2].position.y,
            self.top_center_line.vertices[0].position.x, self.top_center_line.vertices[0].position.y,
            self.top_center_line.vertices[self.top_center_line.vertices.len() - 1].position.x, self.top_center_line.vertices[self.top_center_line.vertices.len() - 1].position.y,
            self.mid_left_line.vertices[0].position.x, self.mid_left_line.vertices[0].position.y,
            self.mid_left_line.vertices[self.mid_left_line.vertices.len() - 1].position.x, self.mid_left_line.vertices[self.mid_left_line.vertices.len() - 1].position.y,
            self.bottom_left_line.vertices[0].position.x, self.bottom_left_line.vertices[0].position.y,
            self.bottom_left_line.vertices[self.bottom_left_line.vertices.len() - 1].position.x, self.bottom_left_line.vertices[self.bottom_left_line.vertices.len() - 1].position.y,
            self.bottom_right_line.vertices[0].position.x, self.bottom_right_line.vertices[0].position.y,
            self.bottom_right_line.vertices[self.bottom_right_line.vertices.len() - 1].position.x, self.bottom_right_line.vertices[self.bottom_right_line.vertices.len() - 1].position.y,
            self.mid_right_line.vertices[0].position.x, self.mid_right_line.vertices[0].position.y,
            self.mid_right_line.vertices[self.mid_right_line.vertices.len() - 1].position.x, self.mid_right_line.vertices[self.mid_right_line.vertices.len() - 1].position.y,
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
