use crate::Entity;
use crate::comps::temp_comps::*;
use crate::world::*;

use std::collections::HashMap;
use std::fs;

use serde_yaml::Value;
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
pub fn read_templates() -> HashMap<String, Value> {
    let file = fs::File::open("./src/entity_templates.yaml").unwrap();
    let data: HashMap<String, Value> = serde_yaml::from_reader(file).unwrap();
    data
}

pub fn spawn(
    w: &mut World,
    templates: &HashMap<String, Value>,
    entity_type: &str,
    x: i32,
    y: i32,
) -> Entity {
    let entity = w.add_entity();

    let components = templates.get(entity_type).unwrap();

    for (comp_name, comp_data) in components.as_mapping().unwrap() {
        match comp_name.as_str().unwrap() {
            "Position" => w.add_component(entity, Position { x, y }),

            "Hp" => {
                let value = comp_data["val"].as_i64().unwrap() as i32;
                w.add_component(entity, Hp { val: value });
            }

            "Renderable" => {
                let glyph = comp_data["glyph"].as_str().unwrap().chars().next().unwrap();
                w.add_component(entity, Renderable { glyph: glyph });
            }

            "Hitbox" => w.add_component(entity, Hitbox {}),

            "Player" => w.add_component(entity, Player {}),

            _ => (),
        }
    }
    entity
}
