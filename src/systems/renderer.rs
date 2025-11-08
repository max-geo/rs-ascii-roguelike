use crate::world::*;
use crate::{Position, Renderable};
use tcod::colors::*;
use tcod::console::{Console, Root};

pub fn show_screen(terminal: &mut Root, w: &World) {
    terminal.set_default_background(BLACK);
    terminal.clear();

    let renderables = w.get_entities::<Renderable>();

    for r in renderables {
        terminal.put_char(
            w.get_component::<Position>(r).x,
            w.get_component::<Position>(r).y,
            w.get_component::<Renderable>(r).glyph,
            tcod::BackgroundFlag::None,
        );
    }

    terminal.flush();
}
