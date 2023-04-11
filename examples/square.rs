use endless::graphics::*;

fn main() {
        let square = Square::new([0.0, 0.0, 0.0].into(), 1.0 );

        let _ = pollster::block_on(
                run(
                        square.subdivide(7).mesh
                )
        );
}
