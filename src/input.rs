use tcod::console::Root;
use tcod::input::*;

pub fn handle_input(terminal: &mut Root) -> bool {
    let key = terminal.wait_for_keypress(true);
    match key.code {
        KeyCode::Escape => true,

        _ => false,
    }
}
