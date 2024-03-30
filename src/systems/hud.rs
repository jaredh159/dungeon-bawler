use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(world: &SubWorld) {
  let mut health_query = <&Health>::query().filter(component::<Player>());
  let player_health = health_query.iter(world).next().unwrap();
  let mut draw_batch = DrawBatch::new();
  draw_batch.target(2);

  draw_batch.print_color_centered(
    2,
    "Hey I'm Hairlip. Vim keys to move.",
    ColorPair::new(GREEN, BLACK),
  );

  draw_batch.bar_horizontal(
    Point::zero(),
    SCREEN_WIDTH * 2,
    player_health.current,
    player_health.max,
    ColorPair::new(DARK_RED, BLACK),
  );

  draw_batch.print_color_centered(
    0,
    // macro = shortcut to more code
    format!(
      " Hey your health is {} / {} ", // {} = "placeholder"
      player_health.current, player_health.max
    ),
    ColorPair::new(GREEN, DARK_RED),
  );

  let player = <(Entity, &Player)>::query()
    .iter(world)
    .map(|(entity, _)| *entity)
    .next()
    .unwrap();

  let mut item_query = <(&Item, &Name, &Carried)>::query();
  let mut y = 3;
  item_query
    .iter(world)
    .filter(|(_, _, carried)| carried.0 == player)
    .for_each(|(_, name, _)| {
      draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
      y += 1;
    });
  if y > 3 {
    draw_batch.print_color(
      Point::new(3, 2),
      "Items carried",
      ColorPair::new(YELLOW, BLACK),
    );
  }
  draw_batch.submit(10000).expect("Bachelor error");
}
