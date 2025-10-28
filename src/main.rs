mod comps;
mod entities;
mod input;
mod systems;

use comps::temp_comps::Position;
use entities::temp_entities::Player;
use systems::renderer;
use tcod::console::{FontLayout, Root};

fn main() {
    const WIDTH: i32 = 80;
    const HEIGHT: i32 = 30;

    let mut term = Root::initializer()
        .size(WIDTH, HEIGHT)
        .font("assets/terminal16x16.png", FontLayout::AsciiInRow)
        .init();

    let mut player = Player {
        pos: Position { x: 8, y: 8 },
        glyph: '@',
    };

    loop {
        renderer::show_screen(&mut term, &player);
        if input::handle_input(&mut term) {
            break; //  handle_input returns true on 'escape', false otherwise
        }
    }
}
