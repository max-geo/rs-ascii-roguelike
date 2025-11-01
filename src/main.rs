mod comps;
mod entities;
mod systems;
mod world;

use comps::temp_comps::Component;
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
    let mut player = Entity {
        components: Vec::<Component>::new(),
        id: 22,
    };

    let mut wrld = World::new();
    wrld.add_entity(player);
    println!("{}", wrld.entities[0].id);
    loop {
        renderer::show_screen(&mut term);
        if input::handle_input(&mut term) {
            break; //  handle_input returns true on 'escape', false otherwise
        }
    }
}
