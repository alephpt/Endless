use endless::graphics::*;

fn main() {
        let origin = Position::new(0.0, 0.0, 0.0, 1.0);
        let size = 1.0;
        let geometry = Geometry::new(origin, size, Shape::Square);

        let _ = pollster::block_on(run(geometry));
}
