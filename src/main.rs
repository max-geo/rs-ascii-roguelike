mod comps;
mod entities;
mod systems;
mod utils;
mod world;

use comps::temp_comps::*;
use entities::temp_entities::Entity;
use systems::{input, renderer};
use tcod::console::{FontLayout, Root};
use world::register_comps;
use world::*;

fn main() {
    const WIDTH: i32 = 80;
    const HEIGHT: i32 = 30;

    let mut term = Root::initializer()
        .title("the fabulous land of Paladin")
        .size(WIDTH, HEIGHT)
        .font("assets/terminal16x16.png", FontLayout::AsciiInRow)
        .init();

    //TODO: implementat furia bardiana

    let mut w = World::new();

    register_comps!(w, Position, Renderable, Player, Hp, Damage, Hitbox);

    let player = Entity { id: 22 };
    w.add_entity(player);

    add_comps!(
        w,
        player,
        Position { x: 13, y: 2 },
        Player {},
        Renderable { glyph: '@' },
        Hitbox {}
    );

    let creatura = Entity { id: 21 };
    w.add_entity(creatura);
    add_comps!(
        w,
        creatura,
        Position { x: 15, y: 3 },
        Renderable { glyph: 'B' },
        Hitbox {}
    );

    loop {
        renderer::show_screen(&mut term, &w);
        if input::handle_input(&mut term, &mut w) {
            break; //  handle_input returns true on 'escape', false otherwise
        };
    }
}
