use crate::prelude::*;
use automata::ConwaysGameOfLifeArchitect;
use drunkard::DrunkardsWalkArchitect;
use empty::EmptyArchitect;
use rooms::RoomsArchitect;

use self::prefab::apply_prefab;

const NUM_ROOMS: usize = 20;

mod automata;
mod drunkard;
mod empty;
mod prefab;
mod rooms;

trait MapArchitect {
  fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub struct MapBuilder {
  pub map: Map,
  pub rooms: Vec<Rect>,
  pub monster_spawns: Vec<Point>,
  pub player_start: Point,
  pub toothpaste_start: Point,
}

impl Default for MapBuilder {
  fn default() -> MapBuilder {
    MapBuilder {
      map: Map::new(),
      rooms: Vec::new(),
      monster_spawns: Vec::new(),
      player_start: Point::zero(),
      toothpaste_start: Point::zero(),
    }
  }
}

impl MapBuilder {
  pub fn new(rng: &mut RandomNumberGenerator) -> MapBuilder {
    let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
      0 => Box::new(DrunkardsWalkArchitect {}),
      1 => Box::new(RoomsArchitect {}),
      _ => Box::new(ConwaysGameOfLifeArchitect {}),
    };
    let mut mb = architect.new(rng);
    apply_prefab(&mut mb, rng);
    mb
  }

  pub fn fill_edges(&mut self) {
    self
      .map
      .tiles
      .iter_mut()
      .enumerate()
      .filter(|(idx, _)| {
        let idx: i32 = *idx as i32;
        !(SCREEN_WIDTH..(SCREEN_WIDTH * (SCREEN_HEIGHT - 1))).contains(&idx)
          || idx % SCREEN_WIDTH == 0
          || idx % SCREEN_WIDTH == SCREEN_WIDTH - 1
      })
      .for_each(|(_, tile)| *tile = TileType::Wall)
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
            let idx = map_index(p.x, p.y);
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

  fn find_most_distant(&self) -> Point {
    const UNREACHABLE: &f32 = &f32::MAX;
    let dijkstra_map = DijkstraMap::new(
      SCREEN_WIDTH,
      SCREEN_HEIGHT,
      &[self.map.point2d_to_index(self.player_start)],
      &self.map,
      1024.0,
    );

    let farthest_reachable_index = dijkstra_map
      .map
      .iter() // give them one by one..
      .enumerate() // package them in a tuple (index, value)
      .filter(|(_, dist)| *dist < UNREACHABLE) // pitch unreachable
      .max_by(|a, b| a.1.partial_cmp(b.1).unwrap()) // find biggest
      .unwrap()
      .0;

    self.map.index_to_point2d(farthest_reachable_index)
  }

  fn spawn_monsters(&mut self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
    self.fill_edges();
    const NUM_MONSTERS: usize = 50;
    let mut spawnable_tiles: Vec<Point> = self
      .map
      .tiles
      .iter()
      .enumerate()
      .filter(|(idx, t)| **t == TileType::Floor)
      .filter(|(idx, t)| {
        DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx)) > 10.0
      })
      .map(|(idx, _)| self.map.index_to_point2d(idx))
      .collect();

    let mut spawns = Vec::new();
    for _ in 0..NUM_MONSTERS {
      let random_index = rng.random_slice_index(&spawnable_tiles).unwrap();
      spawns.push(spawnable_tiles[random_index]);
      spawnable_tiles.remove(random_index); // don't want 2 monsters on same tile
    }
    spawns
  }
}
