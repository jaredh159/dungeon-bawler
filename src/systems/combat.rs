use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(world: &mut SubWorld, commands: &mut CommandBuffer) {
  let mut attackers = <(Entity, &WantsToAttack)>::query();
  let victims: Vec<(Entity, Entity, Entity)> = attackers
    .iter(world)
    .map(|(entity, attack)| (*entity, attack.victim, attack.attacker))
    .collect();
  victims.iter().for_each(|(message, victim, attacker)| {
    adjust_health("Victim", victim, 2, world, commands);
    adjust_health("Attacker", attacker, 1, world, commands);
    commands.remove(*message);
  });
}

fn adjust_health(
  name: &'static str,
  entity: &Entity,
  remove_health: i32,
  world: &mut SubWorld,
  commands: &mut CommandBuffer,
) {
  let is_player = world
    .entry_ref(*entity)
    .unwrap()
    .get_component::<Player>()
    .is_ok();
  if let Ok(victim_health) = world
    .entry_mut(*entity)
    .unwrap()
    .get_component_mut::<Health>()
  {
    // now we have the health component of the victim
    // println!("{} health before attack: {}", name, victim_health.current);
    victim_health.current -= remove_health;
    if victim_health.current < 1 && !is_player {
      commands.remove(*entity); // remove dead (non-player) entity
    }
    // println!("{} health after attack: {}", name, victim_health.current);
  }
}
