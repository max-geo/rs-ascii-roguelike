use crate::{
    Entity, World,
    comps::temp_comps::{Component, Equips, Position},
};

pub fn pickup(w: &mut World, player: Entity) -> bool {
    print!("had before: {:?}", w.get_component::<Equips>(player));
    //TODO: MAKE WAND EQUIPPABLE, REMOVE PLAYER UNEQUIPPABLE
    //check_if_inv_space()

    let (player_x, player_y) = (
        w.get_component::<Position>(player).x,
        w.get_component::<Position>(player).y,
    );

    if let Some(item) = w.get_entity_at(player_x, player_y) {
        w.get_component_mut::<Equips>(player).add_value(item);
        w.remove_component::<Position>(item);
    }

    print!("has after: {:?}", w.get_component::<Equips>(player));

    false
}
