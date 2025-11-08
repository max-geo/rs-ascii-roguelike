use crate::comps::temp_comps::{
    AnyComponentStorage, Component, ComponentStorage, ComponentStorageOps, Position,
};
use crate::entities::temp_entities::Entity;

use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct World {
    pub entities: Vec<Entity>,
    entity_count: i32,
    //NOTE: dyn - trait object (dyn Any = object of trait Any)
    //Box moves the actual value to the heap and returns a pointer to the value.
    //this way, the value of dyn Any can actually be stored.
    //this is needed as the size of dyn Any is unknown at compile time.
    storages: HashMap<TypeId, Box<dyn AnyComponentStorage>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::<Entity>::new(),
            entity_count: 0,
            storages: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self) -> Entity {
        let entity = Entity {
            id: self.entity_count,
        };
        self.entities.push(entity);
        self.entity_count += 1;

        entity
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.entities
            .remove(self.entities.iter().position(|e| *e == entity).unwrap());

        for storage in self.storages.values_mut() {
            storage.remove_entity(entity);
        }
    }

    pub fn register_component<T: Component>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.storages.insert(
            type_id,
            Box::new(ComponentStorage::<T>::new()) as Box<dyn AnyComponentStorage>,
        );
    }

    pub fn add_component<T: Component>(&mut self, e: Entity, c: T) {
        let type_id = TypeId::of::<T>();
        let comp_storage = self
            .storages
            .get_mut(&type_id) //NOTE: just get() but returns a mutable reference, but as an Option
            .expect("Component type not registered") //NOTE: expect gets the value inside Option
            .as_any_mut()
            .downcast_mut::<ComponentStorage<T>>()
            .unwrap(); //NOTE: same as expect but without message
        comp_storage.add(e, c)
    }

    pub fn get_component<T: Component>(&self, e: Entity) -> &T {
        let type_id = TypeId::of::<T>();
        self.storages
            .get(&type_id) //NOTE: ? handles None value for the option
            .unwrap()
            .as_any()
            .downcast_ref::<ComponentStorage<T>>()
            .unwrap()
            .get_component(e)
    }

    //TODO: ADD GET ALL COMPONENTS OF ONE ENTITY
    pub fn get_components_of(&self, e: Entity) {}

    pub fn get_entities<T: Component>(&self) -> Vec<Entity> {
        let type_id = TypeId::of::<T>();
        let comp_storage = self
            .storages
            .get(&type_id)
            .unwrap()
            .as_any()
            .downcast_ref::<ComponentStorage<T>>()
            .unwrap();
        comp_storage.get_entities()
    }

    pub fn get_entity_at(&self, x: i32, y: i32) -> Option<Entity> {
        let entities = self.get_entities::<Position>();

        for entity in entities.iter() {
            if self.get_component::<Position>(*entity).x == x
                && self.get_component::<Position>(*entity).y == y
            {
                return Some(*entity);
            }
        }

        None
    }

    pub fn set_component<T: Component>(&mut self, entity: Entity, c: T) {
        let type_id = TypeId::of::<T>();
        let comp_storage = self
            .storages
            .get_mut(&type_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<ComponentStorage<T>>()
            .unwrap();
        comp_storage.set_component(entity, c)
    }
}

macro_rules! register_comps {
    ($w:expr, $ ($comp_name:ty),*) => {
       $($w.register_component::<$comp_name>();)*
    };
}
pub(crate) use register_comps;

macro_rules! add_comps {
    (
        $w:expr,
        $entity:expr,
        $( $comp_name:ident { $( $field_name:ident : $field_val:expr ),* $(,)? } ),* $(,)?
    ) => {{
        $(
            $w.add_component(
                $entity,
                $comp_name {
                    $( $field_name: $field_val ),*
                }
            );
        )*
    }};
}

pub(crate) use add_comps;
