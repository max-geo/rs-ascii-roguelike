mod comps;
mod entities;
mod map;
mod messages;
mod sidescreen;
mod systems;
mod utils;
mod world;

use comps::temp_comps::*;
use entities::temp_entities::Entity;
use map::*;
use messages::*;
use sidescreen::*;
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

    let mut side_screen = SideScreen::new(130, 0, 30, 90, 'c');

    let mut msg_screen = SideScreen::new(0, 79, 131, 11, 'm');

    let screens = vec![&side_screen, &msg_screen];

    let mut w = World::new();

    register_comps!(
        w, Hostile, Position, Renderable, Player, Hp, Damage, Hitbox, Equips, Weapon
    );

    factories::spawn(&mut w, "player", 6, 9);
    factories::spawn(&mut w, "bee", 7, 8);
    factories::spawn(&mut w, "ring", 8, 8);

    // let mut map = generate_map(&term, &mut w);

    generate_map_alt(&term, &mut w);

    let mut messages = Messages::new();
    messages.add_message("cringer".to_string());
    messages.add_message("cgner".to_string());
    messages.add_message("cgner".to_string());
    messages.add_message("8ringer".to_string());

    loop {
        renderer::render(&mut term, &w, &screens, &messages);

        if input::handle_input(&mut term, &mut w, side_screen.width, 10) {
            break; //  handle_input returns true on 'escape', false otherwise
        };
    }
}
