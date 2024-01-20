use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
  entity: &Entity,
  want_move: &WantsToMove,
  #[resource] map: &mut Map,
  #[resource] camera: &mut Camera,
  world: &mut SubWorld,
  commands: &mut CommandBuffer,
) {
  if map.can_enter_tile(want_move.destination) {
    commands.add_component(want_move.entity, want_move.destination);

    if let Ok(entry) = world.entry_ref(want_move.entity) {
      if let Ok(fov) = entry.get_component::<FieldOfView>() {
        commands.add_component(want_move.entity, fov.clone_dirty());

        if entry.get_component::<Player>().is_ok() {
          camera.on_player_move(want_move.destination);
          // everything the player can SEE, goes into his MEMORY fully
          fov.visible_tiles.iter().for_each(|pos| {
            map.memory_tiles[map_index(pos.x, pos.y)] = 255;
          });
          map.memory_tiles.iter_mut().for_each(|memory_strength| {
            if *memory_strength >= MEMORY_LOST_PER_TURN {
              *memory_strength -= MEMORY_LOST_PER_TURN;
            }
          })
        }
      }
    }
  }
  commands.remove(*entity);
}

const MEMORY_LOST_PER_TURN: u8 = 5;
