use crate::{Damage, Entity, Hp, World};

pub fn attack(w: &mut World, attacker: Entity, attacked: Entity) {
    //TODO: add armor, evasion, other effects like crit etc.

    let attacked_hp = w.get_component::<Hp>(attacked);
    let attacker_dmg = w.get_component::<Damage>(attacker);

    let resulting_hp = attacked_hp.val - attacker_dmg.val;

    w.set_component::<Hp>(attacked, Hp { val: resulting_hp });

    print!("attacked");

    if resulting_hp <= 0 {
        w.remove_entity(attacked);
        print!("removed");
    }
}
