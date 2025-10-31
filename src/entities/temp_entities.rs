use crate::comps::temp_comps::{Component, Position};

pub struct Entity {
    components: Vec<Component>,
}

//TODO: add component fn, remove component, search for etc.
impl Entity {
    pub fn new(comp_vec: Vec<Component>) -> Entity {
        Entity {
            components: comp_vec,
        }
    }
}
