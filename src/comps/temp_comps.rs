use crate::Entity;
use std::collections::HashMap;

//PERF: MY BELOVED MACRO
macro_rules! create_comp {
    ($comp_name:ident {$($field_name:ident: $field_val:ty),*}) => {
        impl Component for $comp_name {}
        pub struct $comp_name {
            $(pub $field_name: $field_val),*
        }
    };
    ($comp_name: ident) => {
        impl Component for $comp_name {}
        pub struct $comp_name {}
    };
}

create_comp!(Hitbox);
create_comp!(Player);
create_comp!(Position { x: i32, y: i32 });
create_comp!(Renderable { glyph: char });
create_comp!(Hp { val: i32 });
create_comp!(Damage { val: i32 });

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
