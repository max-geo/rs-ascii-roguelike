use crate::factories::spawn;
use crate::world::World;

use tcod::console::{Console, Root};

pub fn generate_map(t: &Root, w: &mut World) -> Vec<Vec<char>> {
    let mut map = vec![vec!['#'; t.height() as usize]; t.width() as usize];
    for x in 5..t.width() as usize {
        for y in 0..t.height() as usize {
            map[x][y] = ' '
        }
    }
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == '#' {
                spawn(w, "wall", x as i32, y as i32);
            }
        }
    }
    map
}
