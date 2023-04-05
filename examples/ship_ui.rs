use endless::graphics::*;

fn main() {
    let mut ship_ui: ShipUI = ShipUI::new();

    println!("Ship GUI: {}", ship_ui);
    ship_ui.plot();
}
