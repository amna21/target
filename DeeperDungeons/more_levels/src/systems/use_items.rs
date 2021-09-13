//START: header
use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
pub fn use_items(
    ecs: &mut SubWorld, 
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map
) {
    //END: header
    //START: healing_to_apply
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();
    //END: healing_to_apply
    //START: item_activation
    <(Entity, &ActivateItem)>::query().iter(ecs).for_each(|(entity, activate)| {// <callout id="co.use_item.query" />

        let item = ecs.entry_ref(activate.item);// <callout id="co.use_item.entry_ref" />
        if let Ok(item) = item {// <callout id="co.use_item.entry_ref_iflet" />
            if let Ok(healing) = item.get_component::<ProvidesHealing>() {// <callout id="co.use_item.get_component" />
                healing_to_apply.push((activate.used_by, healing.amount));// <callout id="co.use_item.push" />
            }

            if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {// <callout id="co.use_item.dungeonmap" />
                map.revealed_tiles.iter_mut().for_each(|t| *t = true);// <callout id="co.use_item.reveal" />
            }
        }

        commands.remove(activate.item);// <callout id="co.use_item.remove_item" />
        commands.remove(*entity);// <callout id="co.use_item.remove_command" />
    });
    //END: item_activation

    //START: apply_healing
    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {// <callout id="co.use_item.if_let_mut" />
            if let Ok(health) = target.get_component_mut::<Health>() {// <callout id="co.use_item.get_health" />
                health.current = i32::min(health.max, health.current + heal.1);// <callout id="co.use_item.min_health" />
            }
        }
    }
}
//END: apply_healing
