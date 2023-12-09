use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(world: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
  let mut positions = <(Entity, &Point, &Name)>::query();
  let offset = Point::new(camera.left_x, camera.top_y);
  let map_pos = *mouse_pos + offset;
  let mut draw_batch = DrawBatch::new();
  draw_batch.target(2);

  positions
    .iter(world)
    .filter(|(_, pos, _)| **pos == map_pos) // keep if same as mouse pos
    .for_each(|(entity, _, name)| {
      let screen_pos = *mouse_pos * 4; // cuz hud layer is twice as big!
      let display = if let Ok(health) = world.entry_ref(*entity).unwrap().get_component::<Health>()
      {
        // display name and health
        format!("{} : {} Health-Points", &name.0, health.current)
      } else {
        // use just the name
        name.0.clone()
      };
      draw_batch.print(screen_pos, &display);
    });
  draw_batch.submit(10100).expect("Bachelor Error.")
}
