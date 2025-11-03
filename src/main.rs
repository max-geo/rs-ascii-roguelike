mod comps;
mod entities;
mod systems;
mod world;

use comps::temp_comps::{Player, Position, Renderable};
use entities::temp_entities::Entity;
use systems::{input, renderer};
use tcod::console::{FontLayout, Root};
use world::World;

fn main() {
    const WIDTH: i32 = 80;
    const HEIGHT: i32 = 30;

    let mut term = Root::initializer()
        .title("the fabulous land of Paladin")
        .size(WIDTH, HEIGHT)
        .font("assets/terminal16x16.png", FontLayout::AsciiInRow)
        .init();

    //NOTE:fuck you, compiler
    //TODO: implementat furia bardiana

    let mut w = World::new();

    w.register_component::<Position>();
    w.register_component::<Renderable>();
    w.register_component::<Player>();

    let player = Entity { id: 22 };
    w.add_entity(player);
    w.add_component(player, Position { x: 13, y: 2 });
    w.add_component(player, Player {});
    w.add_component(player, Renderable { glyph: '@' });

    let creatura = Entity { id: 21 };
    w.add_entity(creatura);
    w.add_component(creatura, Position { x: 18, y: 2 });
    w.add_component(creatura, Player {});
    w.add_component(creatura, Renderable { glyph: 'B' });
    loop {
        renderer::show_screen(&mut term, &w);
        if input::handle_input(&mut term, &w) {
            break; //  handle_input returns true on 'escape', false otherwise
        };
    }
}
