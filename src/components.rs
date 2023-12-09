use crate::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Render {
  pub color: ColorPair,
  pub glyph: FontCharType,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, PartialEq)]
pub struct WantsToMove {
  pub entity: Entity,
  pub destination: Point,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Health {
  pub current: i32, // u=unsigned integer, 8=8bits 0-255
  pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);
