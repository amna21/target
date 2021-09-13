use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn end_turn(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState
) {
    let mut player_hp = <&Health>::query().filter(component::<Player>());// <callout id="co.losing.pquery" />
    let current_state = turn_state.clone();
    let mut new_state = match current_state {// <callout id="co.losing.mutstate" />
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state // <callout id="co.losing.donothing" />
    };

    player_hp.iter(ecs).for_each(|hp| {// <callout id="co.losing.player_died" />
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
    });

    *turn_state = new_state;
}
