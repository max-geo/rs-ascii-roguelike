mod comps;
mod entities;
mod systems;
mod world;

use comps::temp_comps::{Component, ComponentStorage, Position};
use entities::temp_entities::Entity;
use systems::{input, renderer};
use tcod::console::{FontLayout, Root};
use world::World;

use std::vec;

fn main() {
    const WIDTH: i32 = 80;
    const HEIGHT: i32 = 30;

    let mut term = Root::initializer()
        .title("the fabulous land of Paladin")
        .size(WIDTH, HEIGHT)
        .font("assets/terminal16x16.png", FontLayout::AsciiInRow)
        .init();

    //NOTE:fuck you, compiler
    let player = Entity { id: 22 };

    let mut w = World::new();
    w.add_entity(player);
    //FIX:
    //FIX: COMMENT EVERYTHING THOROUGLY !!!!!!!!!!
    //FIX:
    w.register_component::<Position>();
    w.add_component(player, Position { x: 13, y: 2 });

    loop {
        renderer::show_screen(&mut term, &mut w, player);
        if input::handle_input(&mut term) {
            break; //  handle_input returns true on 'escape', false otherwise
        }
    }
}
