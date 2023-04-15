use endless::graphics::{Sphere, Spherical};
use endless::graphics::Position;
use endless::graphics::run;

fn main() {
    let origin = Position::new(0.0, 0.0, 0.25, 1.0);
    let radius = 0.5;
    let sphere = Sphere::SpherifiedCube;
    let sphere = sphere.new(radius, origin);

    // sphere.subdivide(6);
    // sphere.dedup();

    let _ = pollster::block_on(run(sphere));
}