use tcod::colors::*;
use tcod::console::{Console, Root};

use crate::entities::temp_entities::Player;

pub fn show_screen(terminal: &mut Root, player: &Player) {
    terminal.set_default_background(BLACK);
    terminal.clear();
    terminal.put_char(
        player.pos.x,
        player.pos.y,
        player.glyph,
        tcod::BackgroundFlag::None,
    );
    terminal.flush();
}
