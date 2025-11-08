mod comps;
mod entities;
mod systems;
mod utils;
mod world;

use comps::temp_comps::*;
use entities::temp_entities::Entity;
use systems::{factories, input, renderer};
use tcod::console::{FontLayout, Root};
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

    let templates = factories::read_templates();
    factories::spawn(&mut w, &templates, "player", 3, 3);
    factories::spawn(&mut w, &templates, "bee", 4, 4);

    loop {
        renderer::show_screen(&mut term, &w);
        if input::handle_input(&mut term, &mut w) {
            break; //  handle_input returns true on 'escape', false otherwise
        };
    }
}
