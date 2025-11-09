use crate::Entity;
use std::any::Any;
use std::collections::HashMap;

//PERF: MY BELOVED MACRO
macro_rules! create_comps {
    ( $($comp_name:ident {$($field_name:ident: $field_val:ty),*}),*) => {
       $(
           impl Component for $comp_name {}
        pub struct $comp_name {
            $(pub $field_name: $field_val),*
        }
       )*
    };
}

create_comps!(
    Position { x: i32, y: i32 },
    Renderable { glyph: char },
    Hp { val: i32 },
    Damage { val: i32 },
    Hitbox {},
    Player {},
    ConsumableItem {},
    EquippableItem {},
    Hostile {}
);
#[derive(Debug)]
pub struct Equips {
    pub data: Vec<Entity>,
}

impl Component for Equips {
    fn add_value(&mut self, entity: Entity) {
        self.data.push(entity);
    }
}

// this has to be static for reasons not completely clear to me
// It makes it so it has the longest type of lifetime
pub trait Component: 'static {
    fn add_value(&mut self, entity: Entity) {}
}

pub struct ComponentStorage<T: Component> {
    pub data: HashMap<Entity, T>,
}

impl<T: Component> ComponentStorage<T> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, e: Entity, c: T) {
        self.data.insert(e, c);
    }

    pub fn get_component(&self, e: Entity) -> &T {
        self.data.get(&e).unwrap() //NOTE: get takes a reference here because it does not need ownership of the key
    }

    pub fn get_component_mut(&mut self, e: Entity) -> &mut T {
        self.data.get_mut(&e).unwrap()
    }

    pub fn has_entity(&self, e: Entity) -> bool {
        self.data.contains_key(&e)
    }

    pub fn get_entities(&self) -> Vec<Entity> {
        self.data.keys().cloned().collect()
    }

    pub fn set_component(&mut self, e: Entity, c: T) {
        self.data.insert(e, c);
    }
}

//this is needed since the dyn any approach is limiting
//i cant remove all components of one entity since i have
//to downcast everytime but the type is unknown
pub trait ComponentStorageOps {
    fn remove_entity(&mut self, e: Entity);
}

impl<T: Component> ComponentStorageOps for ComponentStorage<T> {
    fn remove_entity(&mut self, e: Entity) {
        self.data.remove(&e);
    }
}

pub trait AnyComponentStorage: Any + ComponentStorageOps {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Component> AnyComponentStorage for ComponentStorage<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
