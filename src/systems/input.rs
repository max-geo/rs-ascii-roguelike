use crate::world::*;
use crate::{Player, Position};

use tcod::console::Root;
use tcod::input::*;

//TODO: move entity with player component
pub fn handle_input(terminal: &mut Root, w: &mut World) -> bool {
    let key = terminal.wait_for_keypress(true);
    let player = w.get_entities::<Player>();

    let player_x = w.get_component::<Position>(player[0]).x;
    let player_y = w.get_component::<Position>(player[0]).y;

    let mut dx = 0;
    let mut dy = 0;

    match key.code {
        KeyCode::Right => dx += 1,
        KeyCode::Left => dx -= 1,
        KeyCode::Up => dy -= 1,
        KeyCode::Down => dy += 1,

        KeyCode::Escape => return true,

        _ => {}
    }

    w.set_component::<Position>(
        player[0],
        Position {
            x: player_x + dx,
            y: player_y + dy,
        },
    );
    false
}
