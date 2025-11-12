use crate::Entity;
use crate::comps::temp_comps::*;
use crate::world::*;

use std::collections::HashMap;
use std::fs;

use serde_yaml::Value;

pub fn read_templates() -> HashMap<String, Value> {
    let file = fs::File::open("./src/entity_templates.yaml").unwrap();
    let data: HashMap<String, Value> = serde_yaml::from_reader(file).unwrap();

    data
}

pub fn spawn(w: &mut World, entity_type: &str, x: i32, y: i32) -> Entity {
    let templates = read_templates();

    let entity = w.add_entity();

    let components = templates.get(entity_type).unwrap();

    for (comp_name, comp_data) in components.as_mapping().unwrap() {
        match comp_name.as_str().unwrap() {
            "Position" => w.add_component(entity, Position { x, y }),

            "Hp" => {
                let value = comp_data["val"].as_i64().unwrap() as i32;
                w.add_component(entity, Hp { val: value });
            }

            "Damage" => {
                let value = comp_data["val"].as_i64().unwrap() as i32;
                w.add_component(entity, Damage { val: value });
            }

            "Renderable" => {
                let glyph_ch = comp_data["glyph"].as_str().unwrap().chars().next().unwrap();
                w.add_component(entity, Renderable { glyph: glyph_ch });
            }

            "Hitbox" => w.add_component(entity, Hitbox {}),

            "Player" => w.add_component(entity, Player {}),

            "Equips" => w.add_component(
                entity,
                Equips {
                    data: Vec::<Entity>::new(),
                },
            ),

            "Hostile" => w.add_component(entity, Hostile {}),

            "Weapon" => {
                let damage_val = comp_data["damage"].as_i64().unwrap() as i32;
                w.add_component(entity, Weapon { damage: damage_val });
            }

            _ => (),
        }
    }

    entity
}

pub fn spawn_decor(w: &mut World, glyph_ch: char, x_coord: i32, y_coord: i32) -> Entity {
    let entity = w.add_entity();
    w.add_component(entity, Renderable { glyph: glyph_ch });
    w.add_component(
        entity,
        Position {
            x: x_coord,
            y: y_coord,
        },
    );

    entity
}
