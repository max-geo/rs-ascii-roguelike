use crate::Entity;
use std::collections::HashMap;

pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl Position {
    pub fn set_value(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

pub struct Player {}

pub struct Renderable {
    pub glyph: char,
}

impl Component for Position {}
impl Component for Player {}
impl Component for Renderable {}

//NOTE: this has to be static for reasons not completely clear to me
//NOTE: It makes it so it has the longest type of lifetime
pub trait Component: 'static {
    fn set_value(&self) {}
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

    pub fn get_entities(&self) -> Vec<Entity> {
        self.data.keys().cloned().collect()
    }

    pub fn set_component(&mut self, e: Entity, c: T) {
        self.data.insert(e, c);
    }
}
