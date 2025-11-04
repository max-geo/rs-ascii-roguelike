use crate::Entity;

pub fn subtract_vec(v1: Vec<Entity>, v2: Vec<Entity>) -> Vec<Entity> {
    v1.iter().filter(|e| !v2.contains(e)).cloned().collect()
}
