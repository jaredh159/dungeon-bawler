#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod map;
mod map_builder;
mod player;
use prelude::*;

struct State {
  map: Map,
  player: Player,
}

impl State {
  fn new() -> State {
    let mut rng = RandomNumberGenerator::new();
    let map_builder = MapBuilder::new(&mut rng);
    State {
      map: map_builder.map,
      player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
    }
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) {
    ctx.cls(); // erase everything!
    self.player.update(ctx, &self.map);
    self.map.render(ctx);
    self.player.render(ctx);
  }
}

fn main() -> BError {
  let context = BTermBuilder::simple80x50()
    .with_title("Dungeon Bawler")
    .with_fps_cap(30.0)
    .build()?;

  main_loop(context, State::new())
}

mod prelude {
  pub use bracket_lib::prelude::*;
  pub const SCREEN_WIDTH: i32 = 80;
  pub const SCREEN_HEIGHT: i32 = 50;
  pub use crate::map::*;
  pub use crate::map_builder::*;
  pub use crate::player::*;
}
