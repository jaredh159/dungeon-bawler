use crate::prelude::*; // `*` means "all things, or everything"

const COOL_LOOKIN_ROOM: (&str, i32, i32) = (
  "
............
...######...
...#....#...
...#....#...
...#....###.
...#.....M..
...#....###.
...#....#...
...#...#....
....#M.#....
....#.#.....
.....##.....
............
",
  12,
  13,
);

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
  let mut placement: Option<Point> = None;

  let dijkstra_map = DijkstraMap::new(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    &[mb.map.point2d_to_index(mb.player_start)],
    &mb.map,
    1024.0,
  );

  let mut attempts = 0;
  while placement.is_none() && attempts < 10 {
    let possible_placement_rect = Rect::with_size(
      rng.range(0, SCREEN_WIDTH - COOL_LOOKIN_ROOM.1),
      rng.range(0, SCREEN_HEIGHT - COOL_LOOKIN_ROOM.2),
      COOL_LOOKIN_ROOM.1,
      COOL_LOOKIN_ROOM.2,
    );
    let mut can_place = false;
    possible_placement_rect.for_each(|pt| {
      let idx = mb.map.point2d_to_index(pt);
      let distance = dijkstra_map.map[idx];
      if distance < 2000.0 && distance > 20.0 && mb.toothpaste_start != pt {
        can_place = true;
      }
    });
    if can_place {
      placement = Some(Point::new(
        possible_placement_rect.x1,
        possible_placement_rect.y1,
      ));
      let points = possible_placement_rect.point_set();
      mb.monster_spawns.retain(|pt| !points.contains(pt));
    }
    attempts += 1;
  }

  println!("placed: {}", placement.is_some());

  if let Some(placement) = placement {
    let vec_of_chars: Vec<char> = COOL_LOOKIN_ROOM
      .0
      .chars()
      .filter(|a| *a != '\r' && *a != '\n')
      .collect();

    let mut i = 0;
    for ty in placement.y..placement.y + COOL_LOOKIN_ROOM.2 {
      for tx in placement.x..placement.x + COOL_LOOKIN_ROOM.1 {
        let idx = map_index(tx, ty);
        let c = vec_of_chars[i];
        match c {
          'M' => {
            mb.map.tiles[idx] = TileType::Floor;
            mb.monster_spawns.push(Point::new(tx, ty));
          }
          '.' => mb.map.tiles[idx] = TileType::Floor,
          '#' => mb.map.tiles[idx] = TileType::Wall,
          _ => println!("no  idear what to do with the letter {}", c),
        }
        i += 1
      }
    }
  }
}
