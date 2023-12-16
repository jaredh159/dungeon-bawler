use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
  world: &mut SubWorld,
  commands: &mut CommandBuffer,
  #[resource] key: &Option<VirtualKeyCode>,
  #[resource] turn_state: &mut TurnState,
) {
  let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
  let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

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

    let mut did_somethin = false;
    if delta.x != 0 || delta.y != 0 {
      let mut hit_something = false;
      enemies
        .iter(world)
        .filter(|(_, pos)| **pos == destination)
        .for_each(|(entity, _)| {
          // yes we hit something
          hit_something = true;
          did_somethin = true;
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
        did_somethin = true;
        commands.push(((), WantsToMove { entity: player_entity, destination }));
      }
    }

    // !!! TWO MOST IMPORTANT RUST TYPES !!!
    // Result -> 2 case: Ok(thing), Err(err)
    // Option -> 2 case: Some(thing), None

    if !did_somethin {
      if let Ok(health) = world // gives us access to the value inside Result
        .entry_mut(player_entity)
        .unwrap()
        .get_component_mut::<Health>()
      {
        // so inside here, we have the THING in the result
        // heal by not moving, but not higher than max!
        health.current = i32::min(health.max, health.current + 1);
      }
    }
    *turn_state = TurnState::PlayerTurn;
  }
}
