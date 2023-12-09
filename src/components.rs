use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
  pub color: ColorPair,
  pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
  pub entity: Entity,
  pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
  pub current: u8, // u=unsigned integer, 8=8bits 0-255
  pub max: u8,
}
