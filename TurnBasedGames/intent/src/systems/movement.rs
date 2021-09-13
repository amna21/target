use crate::prelude::*;

#[system(for_each)]// <callout id="co.tbs.intent.sys_for_each" />
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);// <callout id="co.tbs.intent.commands" />

        if ecs.entry_ref(want_move.entity)// <callout id="co.tbs.intent.entry_ref" />
            .unwrap()// <callout id="co.tbs.intent.entry_unwrap" />
            .get_component::<Player>().is_ok()// <callout id="co.tbs.intent.get_component_result" />
        {
            camera.on_player_move(want_move.destination);// <callout id="co.tbs.intent.on_player_move" />
        }
    }
    commands.remove(*entity);// <callout id="co.tbs.intent.cmdremove" />
}
