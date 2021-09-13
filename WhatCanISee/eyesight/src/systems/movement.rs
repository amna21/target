use crate::prelude::*;

//START: header
#[system]
#[read_component(WantsToMove)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(FieldOfView)]// <callout id="co.wcis.fov.fovquerymove" />
pub fn movement(
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &SubWorld,
    commands: &mut CommandBuffer
) {
    let mut movers = <(Entity, &WantsToMove)>::query();
    //END: header
    movers.iter(ecs).for_each(| (entity, want_move) | {
        if map.can_enter_tile(want_move.destination) {
            commands.add_component(want_move.entity, want_move.destination);

            //START: body
            if let Ok(entry) = ecs.entry_ref(want_move.entity) {// <callout id="co.wcis.fov.ifentryref" />
                if let Ok(fov) = entry.get_component::<FieldOfView>() {
                    commands.add_component(want_move.entity, fov.clone_dirty());// <callout id="co.wcis.fov.addirty" />
                }

                if entry.get_component::<Player>().is_ok()
                {
                    camera.on_player_move(want_move.destination);
                }
            }
            //END: body
        }
        commands.remove(*entity);
    });
}