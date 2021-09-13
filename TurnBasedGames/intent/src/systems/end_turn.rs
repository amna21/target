use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    let new_state = match turn_state {
        TurnState::AwaitingInput => return,// <callout id="co.tbs.turnbased.await" />
        TurnState::PlayerTurn => TurnState::MonsterTurn,// <callout id="co.tbs.turnbased.player" />
        TurnState::MonsterTurn => TurnState::AwaitingInput// <callout id="co.tbs.turnbased.monster" />
    };

    *turn_state = new_state;// <callout id="co.tbs.turnbased.set" />
}
