use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, PartialEq)]
pub struct Carried(pub Entity);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ProvidesHealing {
  pub amount: i32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Render {
  pub color: ColorPair,
  pub glyph: FontCharType,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ProvidesDungeonMap;

#[derive(Clone, PartialEq, Debug)]
pub struct FieldOfView {
  pub visible_tiles: HashSet<Point>,
  pub radius: i32,
  pub is_dirty: bool,
}

impl FieldOfView {
  pub fn new(radius: i32) -> Self {
    Self {
      visible_tiles: HashSet::new(),
      radius,
      is_dirty: true,
    }
  }

  pub fn clone_dirty(&self) -> Self {
    Self {
      visible_tiles: HashSet::new(),
      radius: self.radius,
      is_dirty: true,
    }
  }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Item;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ToothpasteOfYALT;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Player;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Enemy;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MovingRandomly;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct IntelligentMonster;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
  pub entity: Entity,
  pub destination: Point,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Health {
  pub current: i32, // u=unsigned integer, 32bits
  pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
  pub attacker: Entity,
  pub victim: Entity,
}
