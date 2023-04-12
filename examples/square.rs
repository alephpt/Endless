use endless::graphics::*;

fn main() {
        let origin = Position::new(0.0, 0.0, 0.0, 1.0);
        let size = 1.0;
        let mut quad = Square::new(origin, size);

        quad.subdivide(2);
        quad.dedup();

        let _ = pollster::block_on(run(quad));
}
