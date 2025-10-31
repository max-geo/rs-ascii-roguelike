use crate::entitites::temp_entities::Entity;
use std::vec;

pub struct World {
    entities: Vec<Entity>,
}

impl World {
    pub fn new() -> World {
        World {
            entities: Vec::<Entity>::new(),
        };
    }
}
