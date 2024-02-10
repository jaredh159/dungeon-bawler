use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
  Wall,
  Floor,
}

// this is our map type
pub struct Map {
  pub tiles: Vec<TileType>,
  pub memory_tiles: Vec<u8>,
}

// our own `impl` block, where we add OUR functions
// that is, NOT associated to a TRAIT
impl Map {
  pub fn new() -> Map {
    Map {
      tiles: vec![TileType::Floor; NUM_TILES],
      memory_tiles: vec![0; NUM_TILES],
    }
  }

  pub fn can_enter_tile(&self, point: Point) -> bool {
    self.in_bounds(point) && self.tiles[map_index(point.x, point.y)] == TileType::Floor
  }

  pub fn try_idx(&self, point: Point) -> Option<usize> {
    if !self.in_bounds(point) {
      return None;
    }
    Some(map_index(point.x, point.y))
  }

  pub fn in_bounds(&self, point: Point) -> bool {
    point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
  }

  fn valid_exit(&self, pos: Point, delta: Point) -> Option<usize> {
    let destination = pos + delta;
    if self.in_bounds(destination) {
      if self.can_enter_tile(destination) {
        let index = self.point2d_to_index(destination);
        Some(index)
      } else {
        None
      }
    } else {
      None
    }
  }
}

// here are functions for `BaseMap` TRAIT
impl BaseMap for Map {
  fn is_opaque(&self, idx: usize) -> bool {
    // not sure why we're trapping without the bounds check,
    // hubert doesn't check, probably something we did :/
    if idx >= self.tiles.len() {
      true
    } else {
      self.tiles[idx] != TileType::Floor
    }
  }

  fn get_available_exits(&self, index: usize) -> SmallVec<[(usize, f32); 10]> {
    let mut exits = SmallVec::new();
    let location = self.index_to_point2d(index);
    // test the LEFT tile
    if let Some(index) = self.valid_exit(location, Point::new(-1, 0)) {
      exits.push((index, 1.0));
    }
    //right
    if let Some(index) = self.valid_exit(location, Point::new(1, 0)) {
      exits.push((index, 1.0));
    }
    //top
    if let Some(index) = self.valid_exit(location, Point::new(0, -1)) {
      exits.push((index, 1.0));
    }
    //bottom
    if let Some(index) = self.valid_exit(location, Point::new(0, 1)) {
      exits.push((index, 1.0));
    }
    exits
  }

  fn get_pathing_distance(&self, index1: usize, index2: usize) -> f32 {
    DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(index1), self.index_to_point2d(index2))
  }
}

// here are functions for `Algorithm2D` TRAIT
impl Algorithm2D for Map {
  fn dimensions(&self) -> Point {
    Point::new(SCREEN_WIDTH, SCREEN_WIDTH)
  }

  fn in_bounds(&self, point: Point) -> bool {
    self.in_bounds(point)
  }
}

pub fn map_index(x: i32, y: i32) -> usize {
  (y * SCREEN_WIDTH + x) as usize
}
