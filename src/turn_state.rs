// enumeration "one of N (3) things"

#[derive(Debug, Clone)]
pub enum TurnState {
  AwaitingInput, // case 0
  PlayerTurn,    // case 1
  MonsterTurn,   // case 2
}
