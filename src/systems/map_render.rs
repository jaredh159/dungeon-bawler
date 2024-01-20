use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(world: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
  let mut fov = <&FieldOfView>::query().filter(component::<Player>());
  let mut draw_batch = DrawBatch::new();
  draw_batch.target(0);

  let player_fov = fov.iter(world).next().unwrap();

  for y in camera.top_y..=camera.bottom_y {
    for x in camera.left_x..=camera.right_x {
      let pt = Point::new(x, y);
      let offset = Point::new(camera.left_x, camera.top_y);
      let idx = map_index(x, y);
      // if point is in map AND player can see it OR remember...
      if map.in_bounds(pt) && player_fov.visible_tiles.contains(&pt) | map.memory_tiles[idx] {
        let tint = if player_fov.visible_tiles.contains(&pt) {
          WHITE
        } else {
          (50, 50, 50)
        };
        let glyph = match map.tiles[idx] {
          TileType::Floor => to_cp437('F'),
          TileType::Wall => to_cp437('W'),
        };
        draw_batch.set(pt - offset, ColorPair::new(tint, BLACK), glyph);
      }
    }
  }
  draw_batch.submit(0).expect("Batch error");
}
