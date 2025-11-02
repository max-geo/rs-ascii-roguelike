use crate::Entity;
use std::collections::HashMap;

pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Component for Position {}
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

    pub fn insert(&mut self, e: Entity, c: T) {
        self.data.insert(e, c);
    }

    pub fn get<'a>(&'a self, e: Entity) -> Option<&'a T> {
        self.data.get(&e)
    }
    fn add() {}
}
