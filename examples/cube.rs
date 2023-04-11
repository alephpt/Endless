use endless::graphics::Cube;
use endless::graphics::Position;
use endless::graphics::run;

fn main() {
    let cube = Cube::new(Position::new(0.0, 0.0, 0.0, 1.0), 1.0);
    println!("cube: {}", cube);
    let _ = pollster::block_on(run(cube));
}