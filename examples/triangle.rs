use endless::graphics::*;

fn main() {
    // create a triangle
    let triangle = Triangle::new(Position::new(0.0, 0.0, 0.0, 1.0), 1.0);
    
    let _ = pollster::block_on(run(triangle));
}
