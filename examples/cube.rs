use endless::graphics::Cube;
use endless::graphics::Position;
use endless::graphics::run;

fn main() {
    let mut cube = Cube::new(Position::new(0.0, 0.0, 0.5, 1.0), 1.0);
    cube.subdivide(2);

    let _ = pollster::block_on(run(cube));
}