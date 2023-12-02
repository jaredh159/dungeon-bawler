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
      player_start: Point::zero(),
    };
    mb.fill(TileType::Wall);
    mb.build_random_rooms(rng);
    mb.build_corridors(rng);
    mb.player_start = mb.rooms[0].center();
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

  fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
    use std::cmp::{max, min};
    for y in min(y1, y2)..=max(y1, y2) {
      if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
        self.map.tiles[idx] = TileType::Floor;
      }
    }
  }

  fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
    use std::cmp::{max, min};
    for x in min(x1, x2)..=max(x1, x2) {
      if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
        self.map.tiles[idx] = TileType::Floor;
      }
    }
  }

  fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
    let mut rooms = self.rooms.clone();
    rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
    for (i, room) in rooms.iter().enumerate().skip(1) {
      let prev_center = rooms[i - 1].center();
      let curr_center = room.center();
      if rng.range(0, 2) == 1 {
        self.apply_horizontal_tunnel(prev_center.x, curr_center.x, prev_center.y);
        self.apply_vertical_tunnel(prev_center.y, curr_center.y, curr_center.x);
      } else {
        self.apply_vertical_tunnel(prev_center.y, curr_center.y, prev_center.x);
        self.apply_horizontal_tunnel(prev_center.x, curr_center.x, curr_center.y);
      }
    }
  }

  fn fill(&mut self, tile_type: TileType) {
    self.map.tiles.iter_mut().for_each(|t| *t = tile_type);
  }
}
