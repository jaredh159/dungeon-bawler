use crate::prelude::*;

pub fn spawn_player(world: &mut World, pos: Point) {
  world.push((
    Player,
    pos,
    Render {
      color: ColorPair::new(WHITE, BLACK),
      glyph: to_cp437('@'),
    },
    Health { current: 4, max: 4 },
    Name("Player".to_string()),
  ));
}

fn goblin() -> (i32, String, FontCharType) {
  (1, "Goblin :)".to_string(), to_cp437('g'))
}

fn huckle_troll() -> (i32, String, FontCharType) {
  (4, "Huckle Troll".to_string(), to_cp437('O'))
}

pub fn spawn_monster(world: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
  let (health, name, letter) = match rng.roll_dice(1, 10) {
    1..=8 => goblin(),
    _ => huckle_troll(),
  };

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
  ));
}
