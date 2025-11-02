use crate::comps::temp_comps::{Component, ComponentStorage};
use crate::entities::temp_entities::Entity;

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::vec;

pub struct World {
    pub entities: Vec<Entity>,
    storages: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::<Entity>::new(),
            storages: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn register_component<T: Component>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.storages
            .insert(type_id, Box::new(ComponentStorage::<T>::new()));
    }

    pub fn add_component<T: Component>(&mut self, e: Entity, c: T) {
        let type_id = TypeId::of::<T>();
        let storage = self
            .storages
            .get_mut(&type_id)
            .expect("Component type not registered")
            .downcast_mut::<ComponentStorage<T>>()
            .unwrap();
        storage.insert(e, c)
    }
    pub fn get_component<T: Component>(&self, e: Entity) -> Option<&T> {
        self.storages
            .get(&TypeId::of::<T>())?
            // .expect("")
            .downcast_ref::<ComponentStorage<T>>()?
            .get(e)
    }
    pub fn set_component<T: Component>(entity: Entity, component: T) {}
}
