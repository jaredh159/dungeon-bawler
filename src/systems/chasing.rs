use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(IntelligentMonster)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, world: &SubWorld, commands: &mut CommandBuffer) {
  let mut movers_query = <(Entity, &Point, &IntelligentMonster)>::query();
  let mut positions_query = <(Entity, &Point, &Health)>::query();
  let mut player_query = <(&Point, &Player)>::query();

  let player_pos = player_query.iter(world).next().unwrap().0;
  let player_pos_index = map_index(player_pos.x, player_pos.y);
  let search_targets = vec![player_pos_index];
  let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

  movers_query.iter(world).for_each(|(entity, pos, _)| {
    let index = map_index(pos.x, pos.y);
    if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, index, map) {
      let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
      let destination = if distance > 1.2 {
        map.index_to_point2d(destination) // take a step closer
      } else {
        *player_pos // attack the player
      };
      let mut attacked = false;
      positions_query
        .iter(world)
        .filter(|(_, target_pos, _)| **target_pos == destination)
        .for_each(|(victim, _, _)| {
          // if the chasing monster hit something, and it is the player...
          if world
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
          {
            commands.push(((), WantsToAttack { attacker: *entity, victim: *victim }));
          }
          attacked = true;
        });
      if !attacked {
        commands.push(((), WantsToMove { entity: *entity, destination }));
      }
    }
  })
}
