mod comps;
mod entities;
mod map;
mod systems;
mod utils;
mod world;

use comps::temp_comps::*;
use entities::temp_entities::Entity;
use map::*;
use systems::{factories, input, renderer};
use tcod::console::{FontLayout, Root};
use world::*;

fn main() {
    const WIDTH: i32 = 160;
    const HEIGHT: i32 = 90;

    let mut term = Root::initializer()
        .title("the fabulous land of Paladin")
        .size(WIDTH, HEIGHT)
        .fullscreen(true)
        .font("assets/terminal16x16.png", FontLayout::AsciiInRow)
        .init();
    //TODO: implementat furia bardiana

    let mut w = World::new();

    register_comps!(
        w, Hostile, Position, Renderable, Player, Hp, Damage, Hitbox, Equips, Weapon
    );

    factories::spawn(&mut w, "player", 6, 9);
    factories::spawn(&mut w, "bee", 7, 8);
    factories::spawn(&mut w, "ring", 8, 8);

    // let mut map = generate_map(&term, &mut w);

    generate_map_alt(&term, &mut w);
    loop {
        renderer::show_screen(&mut term, &w);
        if input::handle_input(&mut term, &mut w) {
            break; //  handle_input returns true on 'escape', false otherwise
        };
    }
}
