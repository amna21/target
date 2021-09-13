#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    //START_HIGHLIGHT
    GameOver
    //END_HIGHLIGHT
}