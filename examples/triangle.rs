use endless::graphics::*;

fn main() {
    // create a triangle
    let origin = Position::new(0.0, 0.0, 0.0, 1.0);
    let size = 1.0;
    let triangle = Geometry::new(origin, size, Shape::Triangle);
    let _ = pollster::block_on(run(triangle));
}
