// import line
use endless::graphics::Line;
use endless::graphics::Vertex;
use endless::graphics::Color;
use endless::graphics::Position;
use endless::graphics::Normal;
use endless::graphics::run;

fn _plot_line(line: Line, screen_width: u32, screen_height: u32) {
    let mut screen = vec![vec![' '; screen_width as usize]; screen_height as usize];

    for vertex in line.mesh.vertices {
        let x = vertex.position.x as usize;
        let y = vertex.position.y as usize;

        screen[y][x] = 'X';
    }

    for row in screen {
        for pixel in row {
            print!("{}", pixel);
        }
        println!();
    }
}

pub fn main () {
    const WINDOW_HEIGHT: u32 = 1200;
    const WINDOW_WIDTH: u32 = 1600;

    let starting_vertex = Vertex::new(
        Position::new(100.0, 100.0, 0.0, 0.0), 
        Color::blue(),
        Normal::new(1.0, 0.0, 1.0)
    );

    let ending_vertex = Vertex::new(
        Position::new(WINDOW_WIDTH as f32 - 100.0, WINDOW_HEIGHT as f32 - 100.0, 0.0, 0.0), 
        Color::blue(),
        Normal::new(0.0, 0.0, 1.0)
    );

    let line = Line::new(
        starting_vertex,
        ending_vertex,
        20.0,        
        20
    );

    let _ = pollster::block_on(run(line.mesh));
}