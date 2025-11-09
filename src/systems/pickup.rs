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

    let item = w.get_entity_at(player_x, player_y).unwrap();
    w.get_component_mut::<Equips>(player).add_value(item);

    // WARNING: MIGHT HAVE TO REFACTOR SET_COMPONENTOR SOMETHING SO YOU CAN ADD AN EQUIPABLE TO THE
    // LIST OF EQUIPS WITHOUT HAVING TO RESTATE ALL ALREADY EQUIPPED ITEMS. MAYBE RETHINK COMPONENT
    // ETC.
    false
}
