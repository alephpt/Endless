use endless::graphics::Cube;
use endless::graphics::Position;
use endless::graphics::run;

fn main() {
    let mut cube = Cube::new(Position::new(0.0, 0.0, 0.25, 1.0), 0.75);
    cube.subdivide(6);
    cube.dedup();

    // print number of verts and indices
    println!("Verts: {}", cube.mesh.vertices.len());
    println!("Indices: {}", cube.mesh.indices.len());

    let _ = pollster::block_on(run(cube));
}