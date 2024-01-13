use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(ToothpasteOfYALT)]
pub fn end_turn(world: &SubWorld, #[resource] turn_state: &mut TurnState) {
  let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
  let mut toothpaste_of_yalt = <&Point>::query().filter(component::<ToothpasteOfYALT>());
  let current_state = turn_state.clone();
  let mut next_state = match current_state {
    TurnState::AwaitingInput => return,
    TurnState::PlayerTurn => TurnState::MonsterTurn,
    TurnState::MonsterTurn => TurnState::AwaitingInput,
    // `_` => match EVERYTHING else
    _ => current_state,
  };

  let toothpaste_pos = toothpaste_of_yalt
    .iter(world)
    .next()
    .expect("couldn't find toothpaste!");

  player_hp.iter(world).for_each(|(hp, player_pos)| {
    if hp.current < 1 {
      // if player dies, game over
      next_state = TurnState::GameOver;
    }
    if player_pos == toothpaste_pos {
      next_state = TurnState::FreshToothpaste;
    }
  });

  *turn_state = next_state;
}
