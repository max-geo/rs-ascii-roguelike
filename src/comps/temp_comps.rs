use crate::Entity;
use std::collections::HashMap;

pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl Component for Position {}

pub struct Renderable {
    pub glyph: char,
}
impl Component for Renderable {}

pub struct Player {}
impl Component for Player {}

pub struct Hitbox {}
impl Component for Hitbox {}

//NOTE: this has to be static for reasons not completely clear to me
//NOTE: It makes it so it has the longest type of lifetime
pub trait Component: 'static {}

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
