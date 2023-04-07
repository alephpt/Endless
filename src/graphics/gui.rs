
use crate::graphics::mesh::{Mesh, Vertex};
use crate::graphics::color::Color;

const SCREEN_WIDTH: u32 = 100;
const SCREEN_HEIGHT: u32 = 50;
const CIRCLE_RADIUS: f32 = 7.0;

#[derive(Debug)]
pub struct ShipUIPoints {
    pub center_point: (f32, f32, f32),
    // pentagonal retical that comes from screens edges to the center
    pub top_center_point: (f32, f32, f32),
    pub mid_left_point: (f32, f32, f32),
    pub bottom_left_point: (f32, f32, f32),
    pub bottom_right_point: (f32, f32, f32),
    pub mid_right_point: (f32, f32, f32),
}

#[derive(Debug)]
pub struct ShipUI {
    pub points: ShipUIPoints,
    pub center_circle: Vec<Vertex>,
    pub top_center_line: Vec<Vertex>,
    pub mid_left_line: Vec<Vertex>,
    pub bottom_left_line: Vec<Vertex>,
    pub bottom_right_line: Vec<Vertex>,
    pub mid_right_line: Vec<Vertex>,
}

#[derive(Debug)]
pub struct ShipUIMesh {
    pub center_circle: Mesh,
    pub top_center_line: Mesh,
    pub mid_left_line: Mesh,
    pub bottom_left_line: Mesh,
    pub bottom_right_line: Mesh,
    pub mid_right_line: Mesh,
}

impl ShipUIMesh {
    pub fn new() -> Self {
        let points = ShipUIPoints::new();

        let subdivision = SCREEN_WIDTH + SCREEN_HEIGHT / 4;
        let thickness = 10.0;

        let center_circle = Mesh::circle_mesh(points.center_point, CIRCLE_RADIUS, Color::blue(), thickness, 20);
        let top_center_line = Mesh::line_mesh(points.top_center_point, points.center_point, thickness, subdivision);
        let mid_left_line = Mesh::line_mesh(points.mid_left_point, points.center_point, thickness, subdivision);
        let bottom_left_line = Mesh::line_mesh(points.bottom_left_point, points.center_point, thickness, subdivision);
        let bottom_right_line = Mesh::line_mesh(points.bottom_right_point, points.center_point, thickness, subdivision);
        let mid_right_line = Mesh::line_mesh(points.mid_right_point, points.center_point, thickness, subdivision);

        Self {
            center_circle,
            top_center_line,
            mid_left_line,
            bottom_left_line,
            bottom_right_line,
            mid_right_line,
        }
    }
}

impl ShipUIPoints {
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
            center_point: center,
            top_center_point: points[0],
            mid_left_point: points[1],
            bottom_left_point: points[2],
            bottom_right_point: points[3],
            mid_right_point: points[4],
        }
    }

    // to ShipUIMesh 


    // draw the ship gui in ascii 
    pub fn plot(&self) {
        let mut screen: Vec<Vec<char>> = vec![vec![' '; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

        // draw the center point
        screen[self.center_point.1 as usize][self.center_point.0 as usize] = 'X';

        // draw the pentagonal pyramid
        screen[self.top_center_point.1 as usize][self.top_center_point.0 as usize] = '1';
        screen[self.mid_left_point.1 as usize][self.mid_left_point.0 as usize] = '2';
        screen[self.bottom_left_point.1 as usize][self.bottom_left_point.0 as usize] = '3';
        screen[self.bottom_right_point.1 as usize][self.bottom_right_point.0 as usize] = '4';
        screen[self.mid_right_point.1 as usize][self.mid_right_point.0 as usize] = '5';

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

        let fifth_r = CIRCLE_RADIUS / 5.0;
        let one_third_r = CIRCLE_RADIUS / 1.3;

        // create the two end points of a line, minus the size of the center circle
        let top_center_line: [Vertex; 2] = [
            Vertex::new([ points.top_center_point.0, points.top_center_point.1, points.top_center_point.2, 1.0, ], [1.0, 1.0, 1.0, 1.0], ),
            Vertex::new([ points.center_point.0, points.center_point.1 - CIRCLE_RADIUS, points.center_point.2, 1.0, ], [1.0, 1.0, 1.0, 1.0], ),
        ];

        let mid_left_line: [Vertex; 2] = [
            Vertex::new([ points.mid_left_point.0, points.mid_left_point.1, points.mid_left_point.2, 1.0, ], [1.0, 1.0, 1.0, 1.0], ),
            Vertex::new([ points.center_point.0 - CIRCLE_RADIUS, points.center_point.1 - fifth_r, points.center_point.2, 1.0, ], [1.0, 1.0, 1.0, 1.0], ),
        ];

        let bottom_left_line: [Vertex; 2] = [
            Vertex::new([ points.bottom_left_point.0, points.bottom_left_point.1, points.bottom_left_point.2, 1.0, ], [1.0, 1.0, 1.0, 1.0], ),
            Vertex::new([ points.center_point.0 - one_third_r, points.center_point.1 + one_third_r, points.center_point.2, 1.0, ], [1.0, 1.0, 1.0, 1.0], ),
        ];

        let bottom_right_line: [Vertex; 2] = [
            Vertex::new([ points.bottom_right_point.0, points.bottom_right_point.1, points.bottom_right_point.2, 1.0, ], [1.0, 1.0, 1.0, 1.0], ),
            Vertex::new([ points.center_point.0 + one_third_r, points.center_point.1 + one_third_r, points.center_point.2, 1.0, ], [1.0, 1.0, 1.0, 1.0], ),
        ];

        let mid_right_line: [Vertex; 2] = [
            Vertex::new([ points.mid_right_point.0, points.mid_right_point.1, points.mid_right_point.2, 1.0, ], [1.0, 1.0, 1.0, 1.0], ),
            Vertex::new([ points.center_point.0 + CIRCLE_RADIUS, points.center_point.1 - fifth_r, points.center_point.2, 1.0, ], [1.0, 1.0, 1.0, 1.0], ),
        ];

        // initialize circlee to be blank vector
        let center_circle: [Vertex; 30] = [Vertex::new([0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0, 1.0]); 30];

        Self {
            points,
            center_circle,
            top_center_line,
            mid_left_line,
            bottom_left_line,
            bottom_right_line,
            mid_right_line,
        }
    }

    // calculate points of the center circle and return it as vec<(i32, i32)>
    pub fn calculate_center_circle(&self) -> Vec<(i32, i32)> {
        let mut circle_vec: Vec<(i32, i32)> = Vec::new();
        let mut angle = 0.0;
        let mut i = 0;

        while angle < 2.0 * std::f32::consts::PI {
            let x = self.points.center_point.0 + CIRCLE_RADIUS * angle.cos();
            let y = self.points.center_point.1 + CIRCLE_RADIUS * angle.sin();

            circle_vec.push((x as i32, y as i32));

            angle += std::f32::consts::PI / 9.0;
            i += 1;
        }

        circle_vec
    }

    // calculate the points of a line
    pub fn calculate_line(&self, start: Vertex, end: Vertex) -> Vec<(i32, i32)> {
        let mut line: Vec<(i32, i32)> = Vec::new();

        // cast start and end points to i32
        let mut start_x = start.position[0] as i32;
        let mut start_y = start.position[1] as i32;
        let end_x = end.position[0] as i32;
        let end_y = end.position[1] as i32;

        // calculate distance between start and end points
        let dx = (end_x - start_x).abs();
        let dy = (end_y - start_y).abs();

        // determine direction of line 
        let sx = if start_x < end_x { 1 } else { -1 };
        let sy = if start_y < end_y { 1 } else { -1 };

        // calculate error value based on distance between points
        let mut err = dx - dy;

        // loop until start point is equal to end point
        loop {
            line.push((start_x, start_y));


            if start_x == end_x && start_y == end_y {
                break;
            }

            // calculate error value for next point in line
            let e2 = 2 * err;

            // if error value is greater than -dy, then move along x axis
            if e2 > -dy {
                err -= dy;
                start_x += sx;
            }

            // if error value is less than dx, then move along y axis
            if e2 < dx {
                err += dx;
                start_y += sy;
            }
        }

        line
    }
    

    // calculate lines and circle and return them as a single vector
    pub fn calculate_points(&self) -> Vec<(i32, i32)> {
        // calculate lines and circle points
        let top_center_line = self.calculate_line(self.top_center_line[0], self.top_center_line[1]);
        let mid_left_line = self.calculate_line(self.mid_left_line[0], self.mid_left_line[1]);
        let bottom_left_line = self.calculate_line(self.bottom_left_line[0], self.bottom_left_line[1]);
        let bottom_right_line = self.calculate_line(self.bottom_right_line[0], self.bottom_right_line[1]);
        let mid_right_line = self.calculate_line(self.mid_right_line[0], self.mid_right_line[1]);
        let center_circle_line = self.calculate_center_circle();

        // combine all lines into one vector
        let mut all_lines: Vec<(i32, i32)> = Vec::new();
        all_lines.extend_from_slice(&top_center_line);
        all_lines.extend_from_slice(&mid_left_line);
        all_lines.extend_from_slice(&bottom_left_line);
        all_lines.extend_from_slice(&bottom_right_line);
        all_lines.extend_from_slice(&mid_right_line);
        all_lines.extend_from_slice(&center_circle_line);

        all_lines
    }

    // plot the ship UI to the screen
    pub fn plot(&self) {
        // create blank list 
        let mut screen: Vec<Vec<char>> = vec![vec![' '; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

        // calculate all the points
        let all_points = self.calculate_points();

        // plot all the points
        for i in all_points {
            screen[i.1 as usize][i.0 as usize] = 'X';
        }

        // plot the center point
        screen[self.points.center_point.1 as usize][self.points.center_point.0 as usize] = 'X';

        // print the screen
        for i in screen {
            for j in i {
                print!("{}", j);
            }
            println!();
        }
    }
}

impl std::fmt::Display for ShipUI {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Ship GUI:
            top_center_line: {:?}
            mid_left_line: {:?}
            bottom_left_line: {:?}
            bottom_right_line: {:?}
            mid_right_line: {:?}",
            self.top_center_line, self.mid_left_line, self.bottom_left_line, self.bottom_right_line, self.mid_right_line
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
