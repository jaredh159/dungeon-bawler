use super::MapArchitect;
use crate::prelude::*;

pub struct ConwaysGameOfLifeArchitect {}

impl MapArchitect for ConwaysGameOfLifeArchitect {
  fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
    println!("ConwaysGameOfLifeArchitect");
    let mut mb = MapBuilder::default();
    self.random_noise_map(rng, &mut mb.map);
    for _ in 0..10 {
      self.generation(&mut mb.map);
    }
    mb.fill_edges();
    let start = self.find_start(&mb.map);
    mb.monster_spawns = mb.spawn_monsters(&start, rng);
    mb.player_start = start;
    mb.toothpaste_start = mb.find_most_distant();
    mb
  }
}

impl ConwaysGameOfLifeArchitect {
  fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
    map.tiles.iter_mut().for_each(|t| {
      let roll = rng.range(0, 100);
      match roll {
        0..=55 => *t = TileType::Wall,
        _ => *t = TileType::Floor,
      }
    });
    self.generation(map);
    self.generation(map);
    self.generation(map);
    self.generation(map);
  }

  fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
    let mut neighbors = 0;
    for iy in -1..=1 {
      for ix in -1..=1 {
        if !(ix == 0 && iy == 0) && map.tiles[map_index(x + ix, y + iy)] == TileType::Wall {
          neighbors += 1;
        }
      }
    }
    neighbors
  }

  fn generation(&mut self, map: &mut Map) {
    let mut new_tiles = map.tiles.clone();
    for y in 1..SCREEN_HEIGHT - 1 {
      for x in 1..SCREEN_WIDTH - 1 {
        let neighbors = self.count_neighbors(x, y, map);
        let idx = map_index(x, y);
        if neighbors > 4 || neighbors == 0 {
          new_tiles[idx] = TileType::Wall;
        } else {
          new_tiles[idx] = TileType::Floor;
        }
      }
    }
    map.tiles = new_tiles;
  }

  fn find_start(&self, map: &Map) -> Point {
    let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
    let closest_point = map
      .tiles
      .iter()
      .enumerate()
      .filter(|(_, t)| **t == TileType::Floor)
      .map(|(idx, _)| {
        (
          idx,
          DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)),
        )
      })
      .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(distance2).unwrap())
      .map(|(idx, _)| idx)
      .unwrap();
    map.index_to_point2d(closest_point)
  }
}
