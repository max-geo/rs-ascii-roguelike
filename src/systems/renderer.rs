use crate::Entity;
use crate::world::*;
use crate::{Player, Position, Renderable};
use tcod::colors::*;
use tcod::console::{Console, Root};

pub fn show_screen(terminal: &mut Root, w: &World) {
    terminal.set_default_background(BLACK);
    terminal.clear();

    let renderables: Vec<Entity> = w
        .get_entities::<Renderable>()
        .into_iter()
        .filter(|e| w.has_component::<Position>(*e))
        .collect();

    let mut player: Option<Entity> = None;

    for r in renderables {
        if w.has_component::<Player>(r) {
            player = Some(r);
            continue;
        }
        terminal.put_char(
            w.get_component::<Position>(r).x,
            w.get_component::<Position>(r).y,
            w.get_component::<Renderable>(r).glyph,
            tcod::BackgroundFlag::None,
        );
    }

    terminal.put_char(
        w.get_component::<Position>(player.unwrap()).x,
        w.get_component::<Position>(player.unwrap()).y,
        w.get_component::<Renderable>(player.unwrap()).glyph,
        tcod::BackgroundFlag::None,
    );

    terminal.flush();
}
