use endless::graphics::Cube;
use endless::graphics::Position;
use endless::graphics::run;

fn main() {
    let origin = Position::new(0.0, 0.0, 0.33, 1.0);
    let size = 0.66;

    let mut cube = Cube::new(origin, size);

    cube.subdivide(6);
    cube.dedup();

    let _ = pollster::block_on(run(cube));
}