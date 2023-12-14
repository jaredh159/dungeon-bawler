use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input(
  world: &mut SubWorld,
  commands: &mut CommandBuffer,
  #[resource] key: &Option<VirtualKeyCode>,
  #[resource] turn_state: &mut TurnState,
) {
  let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

  if let Some(key) = key {
    let delta = match key {
      VirtualKeyCode::Left | VirtualKeyCode::H => Point::new(-1, 0),
      VirtualKeyCode::Right | VirtualKeyCode::L => Point::new(1, 0),
      VirtualKeyCode::Up | VirtualKeyCode::K => Point::new(0, -1),
      VirtualKeyCode::Down | VirtualKeyCode::J => Point::new(0, 1),
      _ => Point::zero(),
    };

    let (player_entity, destination) = players
      .iter(world)
      .map(|(entity, pos)| (*entity, *pos + delta))
      .next()
      .unwrap();

    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    if delta.x != 0 || delta.y != 0 {
      let mut hit_something = false;
      enemies
        .iter(world)
        .filter(|(_, pos)| **pos == destination)
        .for_each(|(entity, _)| {
          // yes we hit something
          hit_something = true;
          // say that we want to ATTACK
          commands.push((
            (),
            WantsToAttack {
              attacker: player_entity,
              victim: *entity,
            },
          ));
        });

      // if we didn't hit, say that we want to MOVE
      if !hit_something {
        commands.push(((), WantsToMove { entity: player_entity, destination }));
      }
    }
    *turn_state = TurnState::PlayerTurn;
  }
}
