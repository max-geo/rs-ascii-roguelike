use crate::Entity;
use std::collections::HashMap;

pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Component for Position {}
//NOTE: this has to be static for reasons not completely clear to me
//NOTE: It makes it so it has the longest type of lifetime
pub trait Component: 'static {}

pub struct ComponentStorage<T: Component> {
    data: HashMap<Entity, T>,
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

    pub fn get(&self, e: Entity) -> Option<&T> {
        self.data.get(&e) //NOTE: get takes a reference here because it does not need ownership of the key
    }
}
