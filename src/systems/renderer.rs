use crate::world::*;
use crate::{Entity, Position};
use tcod::colors::*;
use tcod::console::{Console, Root};
// use crate::entities::temp_entities::Entity;
// TODO: render all entities with 'renderable' component
pub fn show_screen(terminal: &mut Root, w: &World, e: Entity) {
    terminal.set_default_background(BLACK);
    terminal.clear();
    // terminal.put_char(
    //     w.storages
    //     .tcod::BackgroundFlag::None,
    // );
    terminal.put_char(
        w.get_component::<Position>(e).unwrap().x,
        w.get_component::<Position>(e).unwrap().y,
        '@',
        tcod::BackgroundFlag::None,
    );
    terminal.flush();
}
