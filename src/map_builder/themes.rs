use crate::prelude::*;

pub struct HubertsDungeonTheme {}

impl HubertsDungeonTheme {
  pub fn new() -> Box<dyn MapTheme> {
    Box::new(HubertsDungeonTheme {})
  }
}

impl MapTheme for HubertsDungeonTheme {
  fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
    match tile_type {
      TileType::Floor => to_cp437('.'),
      TileType::Wall => to_cp437('#'),
    }
  }
}

impl MapTheme for HucksDungeonTheme {
  fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
    match tile_type {
      TileType::Floor => to_cp437(';'),
      TileType::Wall => to_cp437('"'),
    }
  }
}

pub struct HucksDungeonTheme {}

impl HucksDungeonTheme {
  pub fn new() -> Box<dyn MapTheme> {
    Box::new(Self {})
  }
}

pub struct ForestTheme {}

impl MapTheme for ForestTheme {
  fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
    match tile_type {
      TileType::Floor => to_cp437('F'),
      TileType::Wall => to_cp437('W'),
    }
  }
}

impl ForestTheme {
  pub fn new() -> Box<dyn MapTheme> {
    Box::new(Self {})
  }
}
