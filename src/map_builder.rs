use crate::prelude::*;
const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
  pub map: Map,
  pub rooms: Vec<Rect>,
  pub player_start: Point,
}

impl MapBuilder {
  pub fn new(rng: &mut RandomNumberGenerator) -> MapBuilder {
    let mut mb = MapBuilder {
      map: Map::new(),
      rooms: vec![],
      player_start: Point::zero(), // todo: need to start him somewher
    };
    mb.fill(TileType::Wall);
    mb.build_random_rooms(rng);

    mb
  }

  fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
    while self.rooms.len() < NUM_ROOMS {
      let proposed_room = Rect::with_size(
        rng.range(1, SCREEN_WIDTH - 10),  // x
        rng.range(1, SCREEN_HEIGHT - 10), // y
        rng.range(2, 10),                 // width
        rng.range(2, 10),                 // height
      );
      let mut overlap = false;
      for r in self.rooms.iter() {
        if r.intersect(&proposed_room) {
          overlap = true;
        }
      }
      if !overlap {
        proposed_room.for_each(|p| {
          if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
            let idx = map_idx(p.x, p.y);
            self.map.tiles[idx] = TileType::Floor;
          }
        });
        self.rooms.push(proposed_room);
      }
    }
  }

  fn fill(&mut self, tile_type: TileType) {
    self.map.tiles.iter_mut().for_each(|t| *t = tile_type);
  }
}
