use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
  Wall,
  Floor,
}

pub struct Map {
  pub tiles: Vec<TileType>,
}

impl Map {
  pub fn new() -> Map {
    Map {
      tiles: vec![TileType::Floor; NUM_TILES],
    }
  }

  pub fn render(&self, ctx: &mut BTerm) {
    for y in 0..SCREEN_HEIGHT {
      for x in 0..SCREEN_WIDTH {
        let idx = map_idx(x, y);
        match self.tiles[idx] {
          TileType::Floor => ctx.set(x, y, BLACK, BLACK, to_cp437('.')),
          TileType::Wall => ctx.set(x, y, DARKRED, DARKRED, to_cp437('#')),
        }
      }
    }
  }

  pub fn can_entire_tile(&self, point: Point) -> bool {
    self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
  }

  pub fn try_idx(&self, point: Point) -> Option<usize> {
    if !self.in_bounds(point) {
      return None;
    }
    Some(map_idx(point.x, point.y))
  }

  pub fn in_bounds(&self, point: Point) -> bool {
    point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
  }
}

pub fn map_idx(x: i32, y: i32) -> usize {
  (y * SCREEN_WIDTH + x) as usize
}
