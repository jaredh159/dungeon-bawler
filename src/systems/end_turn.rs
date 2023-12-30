use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn end_turn(world: &SubWorld, #[resource] turn_state: &mut TurnState) {
  let mut player_hp = <&Health>::query().filter(component::<Player>());
  let current_state = turn_state.clone();
  let mut next_state = match current_state {
    TurnState::AwaitingInput => return,
    TurnState::PlayerTurn => TurnState::MonsterTurn,
    TurnState::MonsterTurn => TurnState::AwaitingInput,
    // `_` => match EVERYTHING else
    _ => current_state,
  };
  player_hp.iter(world).for_each(|hp| {
    if hp.current < 1 {
      // if player dies, game over
      next_state = TurnState::GameOver;
    }
  });
  *turn_state = next_state;
}
