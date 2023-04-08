use endless::graphics::*;

fn main() {
    // create a triangle
    let mut triangle = Triangle::new(
        Vertex::new(Position::new(-0.5, -0.5, 0.0, 1.0), Color::red(), Normal::new(0.0, 0.0, 1.0)),
        Vertex::new(Position::new(0.5, -0.5, 0.0, 1.0), Color::green(), Normal::new(0.0, 0.0, 1.0)),
        Vertex::new(Position::new(0.0, 0.5, 0.0, 1.0), Color::blue(), Normal::new(0.0, 0.0, 1.0)),
    );

    let _ = pollster::block_on(run(triangle.mesh));
}
