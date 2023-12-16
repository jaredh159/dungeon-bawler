use crate::prelude::*;

fn adjust_health(
  name: &'static str,
  victim: &Entity,
  remove_health: i32,
  world: &mut SubWorld,
  commands: &mut CommandBuffer,
) {
  if let Ok(victim_health) = world
    .entry_mut(*victim)
    .unwrap()
    .get_component_mut::<Health>()
  {
    // now we have the health component of the victim
    println!("{} health before attack: {}", name, victim_health.current);
    victim_health.current -= remove_health;
    if victim_health.current < 1 {
      commands.remove(*victim);
    }
    println!("{} health after attack: {}", name, victim_health.current);
  }
}

#[system]
#[read_component(WantsToAttack)]
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
