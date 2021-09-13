use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
//START: add_turn_state
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
    //START_HIGHLIGHT
    #[resource] turn_state: &mut TurnState
    //END_HIGHLIGHT
)
{
//END: add_turn_state

    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query()
                .filter(component::<Player>());
            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                //START: enter_tile
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                    // START_HIGHLIGHT
                    *turn_state = TurnState::PlayerTurn;
                    // END_HIGHLIGHT
                }
                //END: enter_tile
            });
        }
    }
}
