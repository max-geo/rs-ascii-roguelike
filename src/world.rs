use crate::comps::temp_comps::{Component, ComponentStorage};
use crate::entities::temp_entities::Entity;

use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct World {
    pub entities: Vec<Entity>,
    //NOTE: dyn - trait object (dyn Any = object of trait Any)
    //Box moves the actual value to the heap and returns a pointer to the value.
    //this way, the value of dyn Any can actually be stored.
    //this is needed as the size of dyn Any is unknown at compile time.
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

    // add component to entity
    pub fn add_component<T: Component>(&mut self, e: Entity, c: T) {
        let type_id = TypeId::of::<T>();
        let comp_storage = self
            .storages
            .get_mut(&type_id) //NOTE: just get() but returns a mutable reference, but as an Option
            .expect("Component type not registered") //NOTE: expect gets the value inside Option
            .downcast_mut::<ComponentStorage<T>>()
            .unwrap(); //NOTE: same as expect but without message
        comp_storage.add(e, c)
    }

    // get component T for entity
    pub fn get_component<T: Component>(&self, e: Entity) -> &T {
        let type_id = TypeId::of::<T>();
        self.storages
            .get(&type_id) //NOTE: ? handles None value for the option
            .unwrap()
            .downcast_ref::<ComponentStorage<T>>()
            .unwrap()
            .get_component(e)
    }

    pub fn get_entities<T: Component>(&self) -> Vec<Entity> {
        let type_id = TypeId::of::<T>();
        let comp_storage = self
            .storages
            .get(&type_id)
            .unwrap()
            .downcast_ref::<ComponentStorage<T>>()
            .unwrap();
        comp_storage.get_entities()
    }
    pub fn set_component<T: Component>(&mut self, entity: Entity, c: T) {
        let type_id = TypeId::of::<T>();
        let comp_storage = self
            .storages
            .get_mut(&type_id)
            .unwrap()
            .downcast_mut::<ComponentStorage<T>>()
            .unwrap();
        comp_storage.set_component(entity, c)
    }
}
