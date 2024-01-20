use legion::storage::IntoComponentSource;

use crate::prelude::*;

pub fn spawn_player(world: &mut World, pos: Point) {
  world.push((
    Player,
    pos,
    Render {
      color: ColorPair::new(WHITE, BLACK),
      glyph: to_cp437('P'),
    },
    Health { current: 10, max: 10 },
    Name("Player".to_string()),
    FieldOfView::new(8),
  ));
}

#[allow(non_snake_case)]
pub fn spawn_toothpaste_of_YALT(world: &mut World, pos: Point) {
  world.push((
    Item,
    ToothpasteOfYALT,
    pos,
    Render {
      color: ColorPair::new(WHITE, BLACK),
      glyph: to_cp437('T'),
    },
    Name("Toothpaste of YALT |:^(".to_string()),
  ));
}

fn drunken_goblin() -> (i32, String, FontCharType, bool) {
  (1, "Goblin :)".to_string(), to_cp437('G'), true)
}

fn goblin() -> (i32, String, FontCharType, bool) {
  (1, "Goblin :)".to_string(), to_cp437('G'), false)
}

fn huckle_troll() -> (i32, String, FontCharType, bool) {
  (4, "Huckle Troll".to_string(), to_cp437('O'), false)
}

pub fn spawn_monster(world: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
  let (health, name, letter, has_wine) = match rng.roll_dice(1, 10) {
    1..=7 => goblin(),
    8 => drunken_goblin(),
    _ => huckle_troll(),
  };

  if has_wine {
    world.push((
      Enemy,
      pos,
      Render {
        color: ColorPair::new(WHITE, BLACK),
        glyph: letter,
      },
      MovingRandomly {},
      Health { current: health, max: health },
      Name(name),
      FieldOfView::new(6),
    ));
  } else {
    world.push((
      Enemy,
      pos,
      Render {
        color: ColorPair::new(WHITE, BLACK),
        glyph: letter,
      },
      IntelligentMonster {},
      Health { current: health, max: health },
      Name(name),
      FieldOfView::new(6),
    ));
  }
}
