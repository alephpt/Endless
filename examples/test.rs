use endless::graphics::*;

fn main() {
    let _ = pollster::block_on(
        run(
            Square::new(
            Vertex::new(
                    Position::new(-0.5, 0.5, 0.0, 1.0), 
                    Color::black(), 
                    Normal::new(0.0, 0.0, 1.0)),
            Vertex::new(
                    Position::new(-0.5, -0.5, 0.0, 1.0), 
                    Color::cyan(), 
                    Normal::new(0.0, 0.0, 1.0)),
            Vertex::new(
                    Position::new(0.5, 0.5, 0.0, 1.0), 
                    Color::yellow(), 
                    Normal::new(0.0, 0.0, 1.0)),
            Vertex::new(
                    Position::new(0.5, -0.5, 0.0, 1.0), 
                    Color::magenta(), 
                    Normal::new(0.0, 0.0, 1.0)),

            ).mesh
        )
    );
}
