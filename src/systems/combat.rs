use crate::prelude::*;

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
    // if the victim has a health component, grab it
    if let Ok(victim_health) = world
      .entry_mut(*victim)
      .unwrap()
      .get_component_mut::<Health>()
    {
      // now we have the health component of the victim
      println!("Victim health before attack: {}", victim_health.current);
      victim_health.current -= 2;
      if victim_health.current < 1 {
        commands.remove(*victim);
      }
      println!("Victim health after attack: {}", victim_health.current);
    }
    if let Ok(attacker_health) = world
      .entry_mut(*victim)
      .unwrap() // might crash?
      .get_component_mut::<Health>()
    {
      // now we have the health component of the victim
      println!("Attacker health before attack: {}", attacker_health.current);
      attacker_health.current -= 1;
      if attacker_health.current < 1 {
        commands.remove(*victim);
      }
      println!("Attacker health after attack: {}", attacker_health.current);
    }
    commands.remove(*message);
  });
}
