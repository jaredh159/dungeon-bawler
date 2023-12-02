#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod map;
use prelude::*;

struct State {
  map: Map,
}

impl State {
  fn new() -> State {
    State { map: Map::new() }
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    self.map.render(ctx);
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
}
