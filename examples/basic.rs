extern crate psat_orb;
extern crate psat;

use psat_orb::orbtk;

fn main() {
    let window = orbtk::Window::new(orbtk::Rect::new(100, 100, 400, 400), "psat-orb basic example");
    let mut window = psat_orb::OrbWindow { window };

    let node = psat::h(psat_orb::BUTTON, psat_orb::ButtonProps {text: "a button".to_owned()}, vec![]);

    psat::render(&mut window, &node);

    window.window.exec();
}
