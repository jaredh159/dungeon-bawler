use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
  world: &mut SubWorld,
  #[resource] map: &Map,
  #[resource] key: &Option<VirtualKeyCode>,
  #[resource] camera: &mut Camera,
  #[resource] turn_state: &mut TurnState,
) {
  if let Some(key) = key {
    let delta = match key {
      VirtualKeyCode::Left | VirtualKeyCode::H => Point::new(-1, 0),
      VirtualKeyCode::Right | VirtualKeyCode::L => Point::new(1, 0),
      VirtualKeyCode::Up | VirtualKeyCode::K => Point::new(0, -1),
      VirtualKeyCode::Down | VirtualKeyCode::J => Point::new(0, 1),
      _ => Point::zero(),
    };
    if delta.x != 0 || delta.y != 0 {
      let mut players = <&mut Point>::query().filter(component::<Player>());
      players.iter_mut(world).for_each(|pos| {
        let destination = *pos + delta;
        if map.can_enter_tile(destination) {
          *pos = destination;
          camera.on_player_move(destination);
          *turn_state = TurnState::PlayerTurn;
        }
      });
    }
  }
}
