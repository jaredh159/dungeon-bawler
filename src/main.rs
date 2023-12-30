#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod camera;
mod components;
mod map;
mod map_builder;
mod scratch;
mod spawner;
mod systems;
mod turn_state;
use prelude::*;

struct State {
  world: World, // entity component system
  resources: Resources,
  input_systems: Schedule,
  player_systems: Schedule,
  monster_systems: Schedule,
}

impl State {
  fn new() -> State {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut rng = RandomNumberGenerator::new();
    let map_builder = MapBuilder::new(&mut rng);
    spawn_player(&mut world, map_builder.player_start);
    map_builder
      .rooms
      .iter()
      .skip(1)
      .map(|r| r.center())
      .for_each(|pos| {
        spawn_monster(&mut world, &mut rng, pos);
      });
    resources.insert(TurnState::AwaitingInput);
    resources.insert(map_builder.map);
    resources.insert(Camera::new(map_builder.player_start));
    State {
      world,
      resources,
      input_systems: build_input_scheduler(),
      player_systems: build_player_scheduler(),
      monster_systems: build_monster_scheduler(),
    }
  }

  fn game_over(&mut self, ctx: &mut BTerm) {
    ctx.set_active_console(3);
    ctx.print_color_centered(2, RED, BLACK, "Your quest took a big bamm ;/");
    ctx.print_color_centered(4, WHEAT, BLACK, "Ouchie!");
    ctx.print_color_centered(
      5,
      HOT_PINK,
      BLACK,
      "the toothpaste of YALT remains unused and even at death you have rotten teeth",
    );
    ctx.print_color_centered(
      9,
      SALMON,
      BLACK,
      "you still have a possebility of obtaining clean teeth",
    );
    ctx.print_color_centered(
      11,
      TURQUOISE,
      BLACK,
      "press SPACE to give a go at clean teeth",
    );

    if let Some(VirtualKeyCode::Space) = ctx.key {
      self.world = World::default();
      self.resources = Resources::default();
      let mut rng = RandomNumberGenerator::new();
      let map_builder = MapBuilder::new(&mut rng);
      spawn_player(&mut self.world, map_builder.player_start);
      spawn_toothpaste_of_YALT(&mut self.world, map_builder.toothpaste_start);
      map_builder
        .rooms
        .iter()
        .skip(1)
        .map(|rect| rect.center())
        .for_each(|pos| spawn_monster(&mut self.world, &mut rng, pos));
      self.resources.insert(map_builder.map);
      self.resources.insert(Camera::new(map_builder.player_start));
      self.resources.insert(TurnState::AwaitingInput);
    }
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) {
    // clear all the layers
    ctx.set_active_console(0);
    ctx.cls();
    ctx.set_active_console(1);
    ctx.cls();
    ctx.set_active_console(2);
    ctx.cls();
    ctx.set_active_console(3);
    ctx.cls();

    // insert key and mouse position
    self.resources.insert(ctx.key);
    ctx.set_active_console(0);
    self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

    let current_turn_state = self.resources.get::<TurnState>().unwrap().clone();
    match current_turn_state {
      TurnState::AwaitingInput => self
        .input_systems
        .execute(&mut self.world, &mut self.resources),
      TurnState::PlayerTurn => self
        .player_systems
        .execute(&mut self.world, &mut self.resources),
      TurnState::MonsterTurn => self
        .monster_systems
        .execute(&mut self.world, &mut self.resources),
      TurnState::GameOver => self.game_over(ctx),
    }
    render_draw_buffer(ctx).expect("Render error");
  }
}

use scratch::*;

fn main() -> BError {
  let context = BTermBuilder::new()
    .with_title("Dungeon Bawler")
    .with_fps_cap(30.0)
    .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
    .with_tile_dimensions(40, 40)
    .with_resource_path("resources/")
    .with_font("dungeon-font.png", 32, 32)
    .with_font("huckle-font.png", 32, 32)
    .with_font("terminal8x8.png", 8, 8)
    .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeon-font.png")
    .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "huckle-font.png")
    .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
    .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png")
    .build()?;

  main_loop(context, State::new())
}

mod prelude {
  pub use bracket_lib::prelude::*;
  pub const SCREEN_WIDTH: i32 = 80;
  pub const SCREEN_HEIGHT: i32 = 50;
  pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
  pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
  pub use crate::camera::*;
  pub use crate::components::*;
  pub use crate::map::*;
  pub use crate::map_builder::*;
  pub use crate::spawner::*;
  pub use crate::systems::*;
  pub use crate::turn_state::*;
  pub use legion::systems::CommandBuffer;
  pub use legion::world::SubWorld;
  pub use legion::*;
}
