use crate::entities::temp_entities::Entity;
use std::vec;

pub struct World {
    pub entities: Vec<Entity>,
}

impl World {
    pub fn new() -> World {
        World {
            entities: Vec::<Entity>::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}
