mod input;

use tcod::console::{FontLayout, Root};

fn main() {
    let mut term = Root::initializer()
        .font("assets/terminal16x16.png", FontLayout::Tcod)
        .init();

    loop {
        if input::handle_input(&mut term) {
            break;
        }
    }
}
