use crate::comps::temp_comps::Component;

pub struct Entity {
    pub id: i32,
    pub components: Vec<Component>,
}

//TODO: add component fn, remove component, search for etc.
impl Entity {
    pub fn new(id_nr: i32, comp_vec: Vec<Component>) -> Entity {
        Entity {
            id: id_nr,
            components: comp_vec,
        }
    }
}
