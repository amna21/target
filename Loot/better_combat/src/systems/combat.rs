use crate::prelude::*;

//START: header
#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
//START_HIGHLIGHT
#[read_component(Damage)]
#[read_component(Carried)]
//END_HIGHLIGHT
//END: header
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    //START: victims
    //START_HIGHLIGHT
    let victims : Vec<(Entity, Entity, Entity)> = attackers
    //END_HIGHLIGHT
        .iter(ecs)
        //START_HIGHLIGHT
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim) )
        //END_HIGHLIGHT
        .collect();

    //START_HIGHLIGHT
    victims.iter().for_each(|(message, attacker, victim)| {
    //END_HIGHLIGHT
        //END: victims
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        //START: damage
        let base_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
            if let Ok(dmg) = v.get_component::<Damage>() {
                dmg.0
            } else {
                0
            }
        } else {
            0
        };// <callout id="co.loot.calc_damage" />

        let weapon_damage : i32 = <(&Carried, &Damage)>::query().iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, dmg)| dmg.0)
            .sum();// <callout id="co.loot.weapon_damage" />

        let final_damage = base_damage + weapon_damage;

        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= final_damage;
            //END: damage
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }
        commands.remove(*message);
    });
}