#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    // START_HIGHLIGHT
    NextLevel
    // END_HIGHLIGHT
}