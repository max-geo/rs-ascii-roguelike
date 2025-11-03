use crate::world::*;
use crate::{Position, Renderable};
use tcod::colors::*;
use tcod::console::{Console, Root};
// use crate::entities::temp_entities::Entity;
// TODO: render all entities with 'renderable' component
pub fn show_screen(terminal: &mut Root, w: &World) {
    terminal.set_default_background(BLACK);
    terminal.clear();
    let renderables = w.get_entities::<Renderable>();
    for r in renderables {
        terminal.put_char(
            w.get_component::<Position>(r).unwrap().x,
            w.get_component::<Position>(r).unwrap().y,
            w.get_component::<Renderable>(r).unwrap().glyph,
            tcod::BackgroundFlag::None,
        );
    }
    terminal.flush();
}
