use super::MapArchitect;
use crate::prelude::*;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
  fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
    println!("RoomsArchitect");
    let mut mb = MapBuilder::default();
    mb.fill(TileType::Wall);
    mb.build_random_rooms(rng);
    mb.build_corridors(rng);
    mb.player_start = mb.rooms[0].center();
    mb.toothpaste_start = mb.find_most_distant();
    for room in mb.rooms.iter().skip(1) {
      mb.monster_spawns.push(room.center());
    }
    mb
  }
}
