// import line
use endless::graphics::Line;
use endless::graphics::Vertex;
use endless::graphics::Color;
use endless::graphics::Position;
use endless::graphics::Normal;

fn plot_line(line: Line, screen_width: u32, screen_height: u32) {
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
    let screen_width = 80;
    let screen_height = 50;

    let starting_vertex = Vertex::new(
        Position::new(5.0, 5.0, 0.0, 0.0), 
        Color::blue(),
        Normal::new(1.0, 0.0, 1.0)
    );

    let ending_vertex = Vertex::new(
        Position::new(screen_width as f32 - 5.0, screen_height as f32 - 5.0, 0.0, 0.0), 
        Color::blue(),
        Normal::new(0.0, 0.0, 1.0)
    );

    let line = Line::new(
        starting_vertex,
        ending_vertex,
        4.0,        
        20
    );

    //println!("Line: {}", line);
    plot_line(line, screen_width, screen_height);
}