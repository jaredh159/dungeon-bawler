// enumeration "one of N (3) things"

#[derive(Debug, Clone)]
pub enum TurnState {
  AwaitingInput, // variant 0
  PlayerTurn,    // variant 1
  MonsterTurn,   // variant 2
  GameOver,
  FreshToothpaste,
}
