#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Entity {
    pub id: i32,
}

//TODO: add component fn, remove component, search for etc.
impl Entity {
    pub fn new(id_nr: i32) -> Entity {
        Entity { id: id_nr }
    }
}
