use endless::graphics::Geometry;
use endless::graphics::Shape;
use endless::graphics::Position;
use endless::graphics::run;

fn main() {
    let origin = Position::new(0.0, 0.0, 0.33, 1.0);
    let size = 0.66;

    let mut geometry = Geometry::new(origin, size, Shape::Cube);

    geometry.subdivide(6);
    geometry.dedup();

    // print number of verts and indices
    println!("Verts: {}", geometry.vertex_len());
    println!("Indices: {}", geometry.index_len());

    let _ = pollster::block_on(run(geometry));
}