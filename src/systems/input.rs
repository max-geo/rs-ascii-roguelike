use crate::world::*;

use tcod::console::Root;
use tcod::input::*;

//TODO: move entity with player component
pub fn handle_input(terminal: &mut Root, w: &World) -> bool {
    let key = terminal.wait_for_keypress(true);
    match key.code {
        KeyCode::Escape => true,
        // KeyCode::Up => w.get_entities::<Player>
        _ => false,
    }
}
