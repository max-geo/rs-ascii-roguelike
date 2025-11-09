use crate::systems::{attack::attack, pickup::pickup};
use crate::utils::*;
use crate::world::*;
use crate::{Entity, Hitbox, Player, Position};

use tcod::console::Root;
use tcod::input::*;

pub fn check_collision_area(w: &World, entity: Entity) -> bool {
    let x = w.get_component::<Position>(entity).x;
    let y = w.get_component::<Position>(entity).y;

    let others = w.get_entities::<Hitbox>();

    for other in others {
        let other_x = w.get_component::<Position>(other).x;
        let other_y = w.get_component::<Position>(other).y;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                if other_x == x + dx && other_y == y + dy {
                    return true;
                }
            }
        }
    }

    false
}

pub fn check_collision_at(w: &World, entity: Entity, dx: i32, dy: i32) -> bool {
    let x = w.get_component::<Position>(entity).x;
    let y = w.get_component::<Position>(entity).y;

    let others = subtract_vec(w.get_entities::<Hitbox>(), w.get_entities::<Player>());

    for other in others {
        let other_x = w.get_component::<Position>(other).x;
        let other_y = w.get_component::<Position>(other).y;

        if other_x == x + dx && other_y == y + dy {
            return true;
        }
    }

    false
}
enum Actions {
    Move(i32, i32),
    Pickup,
    Misinput,
    Quit,
}

pub fn handle_input(terminal: &mut Root, w: &mut World) -> bool {
    let key = terminal.wait_for_keypress(true);
    let player = w.get_entities::<Player>()[0];

    let player_x = w.get_component::<Position>(player).x;
    let player_y = w.get_component::<Position>(player).y;

    let action = match key.code {
        KeyCode::NumPad1 => Actions::Move(-1, 1),
        KeyCode::NumPad2 => Actions::Move(0, 1),
        KeyCode::NumPad3 => Actions::Move(1, 1),
        KeyCode::NumPad4 => Actions::Move(-1, 0),
        KeyCode::NumPad5 => Actions::Move(0, 0),
        KeyCode::NumPad6 => Actions::Move(1, 0),
        KeyCode::NumPad7 => Actions::Move(-1, -1),
        KeyCode::NumPad8 => Actions::Move(0, -1),
        KeyCode::NumPad9 => Actions::Move(1, -1),

        KeyCode::Escape => Actions::Quit,

        _ => match key.printable {
            'g' => Actions::Pickup,

            _ => Actions::Misinput,
        },
    };

    match action {
        Actions::Move(dx, dy) => {
            let collides = check_collision_at(w, player, dx, dy);

            if !collides {
                w.set_component::<Position>(
                    player,
                    Position {
                        x: player_x + dx,
                        y: player_y + dy,
                    },
                );
            } else if let Some(colliding_entity) = w.get_entity_at(player_x + dx, player_y + dy) {
                attack(w, player, colliding_entity);
            }

            false
        }

        Actions::Pickup => pickup(w, player),

        Actions::Quit => true,

        Actions::Misinput => false,
    }
}
