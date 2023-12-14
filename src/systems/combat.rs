use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(world: &mut SubWorld, commands: &mut CommandBuffer) {
  let mut attackers = <(Entity, &WantsToAttack)>::query();
  let victims: Vec<(Entity, Entity)> = attackers
    .iter(world)
    .map(|(entity, attack)| (*entity, attack.victim))
    .collect();
  victims.iter().for_each(|(message, victim)| {
    // if the victim has a health component, grab it
    if let Ok(health) = world
      .entry_mut(*victim)
      .unwrap()
      .get_component_mut::<Health>()
    {
      // now we have the health component of the victim
      println!("Health before attack: {}", health.current);
      health.current -= 1;
      if health.current < 1 {
        commands.remove(*victim);
      }
      println!("Health after attack: {}", health.current);
    }
    commands.remove(*message);
  });
}
