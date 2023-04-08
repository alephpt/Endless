use endless::graphics::Ring;
use endless::graphics::Position;
use endless::graphics::Color;

fn plot_ring(ring: Ring, screen_width: u32, screen_height: u32) {
    let mut screen = vec![vec![' '; screen_width as usize]; screen_height as usize];

    for vertex in ring.mesh.vertices {
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

fn main() {
    let screen_width = 75;
    let screen_height = 50;

    let ring = Ring::new(
        Position::new(screen_width as f32 / 2.0, screen_height as f32 / 2.0, 0.0, 0.0),
        17.0,
        7.0,
        17,
        Color::blue()
    );

    plot_ring(ring, screen_width, screen_height);
}